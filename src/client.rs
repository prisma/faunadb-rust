//! Tools for communicating with Fauna.

mod response;

pub use response::*;

use crate::{
    error::{Error, FaunaErrors},
    expr::Expr,
};
use futures::stream::TryStreamExt;
use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{client::HttpConnector, Body, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use serde_json;
use std::{borrow::Cow, time::Duration};
use async_std::future;

type Transport = hyper::Client<HttpsConnector<HttpConnector>>;

/// For building a new Fauna client.
pub struct ClientBuilder<'a> {
    uri: Cow<'a, str>,
    secret: Cow<'a, str>,
    timeout: Duration,
}

impl<'a> ClientBuilder<'a> {
    /// Change the uri if using dedicated Fauna servers. Default:
    /// `https://db.fauna.com`.
    pub fn uri(&mut self, uri: impl Into<Cow<'a, str>>) -> &mut Self {
        self.uri = uri.into();
        self
    }

    /// Request timeout. Default: `60 seconds`.
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Creates the client.
    pub fn build(self) -> crate::Result<Client> {
        let mut builder = hyper::Client::builder();
        builder.keep_alive(true);

        let secret_b64 = base64::encode(&format!("{}:", self.secret));

        Ok(Client {
            transport: builder.build(HttpsConnector::new()?),
            uri: self.uri.parse()?,
            timeout: self.timeout,
            authorization: format!("Basic {}", secret_b64),
        })
    }
}

/// The client for Fauna. Should be created using the
/// [ClientBuilder](struct.ClientBuilder.html).
///
/// Do not create new clients for every request to prevent
/// spamming Fauna servers with new connections.
pub struct Client {
    transport: Transport,
    uri: Uri,
    timeout: Duration,
    authorization: String,
}

impl Client {
    /// Create a new client builder. Secret can be generated in [Fauna Cloud
    /// Console](https://dashboard.fauna.com/keys-new/@db/).
    pub fn builder<'a>(secret: impl Into<Cow<'a, str>>) -> ClientBuilder<'a> {
        ClientBuilder {
            uri: Cow::from("https://db.fauna.com"),
            secret: secret.into(),
            timeout: Duration::new(60, 0),
        }
    }

    /// Send a query to Fauna servers and parsing the response.
    pub async fn query<'a, Q>(&self, query: Q) -> crate::Result<Response>
    where
        Q: Into<Expr<'a>>,
    {
        let query = query.into();
        let payload_json = serde_json::to_string(&query).unwrap();

        trace!("Querying with: {:?}", &payload_json);
        let request = self.request(self.build_request(payload_json));
        let result = future::timeout(self.timeout, request).await??;

        Ok(result)
    }

    async fn request(&self, request: hyper::Request<Body>) -> crate::Result<Response>
    {
        let response = self.transport.request(request).await?;
        trace!("Client::call got response status {}", response.status());

        let status = response.status();
        let body = response.into_body().try_concat().await?;

        match status {
            s if s.is_success() => Ok(serde_json::from_slice(&body).unwrap()),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::BAD_REQUEST => {
                let errors: FaunaErrors = serde_json::from_slice(&body).unwrap();
                Err(Error::BadRequest(errors))
            }
            StatusCode::NOT_FOUND => {
                let errors: FaunaErrors = serde_json::from_slice(&body).unwrap();
                Err(Error::NotFound(errors))
            }
            _ => Err(Error::DatabaseError(String::from_utf8(body.to_vec()).unwrap())),
        }
    }

    fn build_request(&self, payload: String) -> hyper::Request<Body> {
        let mut builder = hyper::Request::builder();

        builder.uri(&self.uri);
        builder.method("POST");

        builder.header(CONTENT_LENGTH, format!("{}", payload.len()).as_bytes());
        builder.header(CONTENT_TYPE, "application/json");
        builder.header(AUTHORIZATION, self.authorization.as_bytes());
        builder.header("X-FaunaDB-API-Version", "2.1");

        builder.body(Body::from(payload)).unwrap()
    }
}
