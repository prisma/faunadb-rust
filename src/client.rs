mod response;

pub use response::*;

use crate::{
    error::{Error, FaunaErrors},
    query::Query,
    FaunaResult,
};
use futures::{future, stream::Stream, Future};
use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{client::HttpConnector, Body, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use serde_json;
use std::time::Duration;
use tokio_timer::Timeout;

type Transport = hyper::Client<HttpsConnector<HttpConnector>>;

pub struct ClientBuilder<'a> {
    uri: &'a str,
    secret: &'a str,
    timeout: Duration,
}

impl<'a> ClientBuilder<'a> {
    pub fn new(secret: &'a str) -> Self {
        Self {
            uri: "https://db.fauna.com",
            secret: secret,
            timeout: Duration::new(60, 0),
        }
    }

    pub fn uri(&mut self, uri: &'a str) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> FaunaResult<Client> {
        let builder = hyper::Client::builder();
        let secret_b64 = base64::encode(&format!("{}:", self.secret));

        Ok(Client {
            transport: builder.build(HttpsConnector::new(1)?),
            uri: self.uri.parse()?,
            timeout: self.timeout,
            authorization: format!("Basic {}", secret_b64),
        })
    }
}

pub struct Client {
    transport: Transport,
    uri: Uri,
    timeout: Duration,
    authorization: String,
}

impl Client {
    pub fn query<'a, Q>(&self, query: Q) -> FutureResponse<Response>
    where
        Q: Into<Query<'a>>,
    {
        let query = query.into();
        let payload_json = serde_json::to_string(&query).unwrap();

        self.request(self.build_request(payload_json), |body| {
            trace!("Got response: {:?}", &body);
            serde_json::from_str(&body).unwrap()
        })
    }

    fn request<F, T>(&self, request: hyper::Request<Body>, f: F) -> FutureResponse<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce(String) -> T + Send + Sync + 'static,
    {
        let send_request = self
            .transport
            .request(request)
            .map_err(|e| Error::ConnectionError(e.into()));

        let requesting = send_request.and_then(move |response| {
            trace!("Client::call got response status {}", response.status());

            let status = response.status();

            let get_body = response
                .into_body()
                .map_err(|e| Error::ConnectionError(e.into()))
                .concat2();

            get_body.and_then(move |body_chunk| {
                if let Ok(body) = String::from_utf8(body_chunk.to_vec()) {
                    match status {
                        s if s.is_success() => future::ok(f(body)),
                        StatusCode::UNAUTHORIZED => future::err(Error::Unauthorized),
                        StatusCode::BAD_REQUEST => {
                            let errors: FaunaErrors = serde_json::from_str(&body).unwrap();
                            future::err(Error::BadRequest(errors))
                        }
                        StatusCode::NOT_FOUND => {
                            let errors: FaunaErrors = serde_json::from_str(&body).unwrap();
                            future::err(Error::NotFound(errors))
                        }
                        _ => future::err(Error::TemporaryFailure(body)),
                    }
                } else {
                    future::err(Error::EmptyResponse)
                }
            })
        });

        let with_timeout = Timeout::new(requesting, self.timeout).map_err(|e| {
            if e.is_timer() {
                Error::TimeoutError
            } else {
                match e.into_inner() {
                    Some(error) => error,
                    None => Error::Other,
                }
            }
        });

        FutureResponse(Box::new(with_timeout))
    }

    fn build_request<'a>(&self, payload: String) -> hyper::Request<Body> {
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
