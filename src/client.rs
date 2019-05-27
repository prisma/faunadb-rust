mod response;

pub use response::*;

use crate::{error::Error, query::Query, FaunaResult};
use futures::{future, Future};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde_json;

type Transport = hyper::Client<HttpsConnector<HttpConnector>>;

pub struct Client {
    transport: Transport,
    host: &'static str,
}

impl Client {
    pub fn new() -> FaunaResult<Self> {
        let builder = hyper::Client::builder();
        let transport = builder.build(HttpsConnector::new(1)?);
        let host = "https://db.fauna.com";

        Ok(Self { transport, host })
    }

    pub fn query<'a, Q>(&self, query: Q) -> FutureResponse<String>
    where
        Q: Into<Query<'a>>,
    {
        let query = query.into();
        let payload_json = serde_json::to_string(&query).unwrap();
        let requesting = future::ok(payload_json);

        FutureResponse(Box::new(requesting))
    }
}
