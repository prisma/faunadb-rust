mod response;

pub use response::*;

use crate::{Error, FaunaResult, Query};
use futures::{future, Future};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

type Transport = hyper::Client<HttpsConnector<HttpConnector>>;

pub struct Client {
    transport: Transport,
    host: &'static str,
}

impl Client {
    fn new() -> FaunaResult<Self> {
        let builder = hyper::Client::builder();
        let transport = builder.build(HttpsConnector::new(1)?);
        let host = "https://db.fauna.com";

        Ok(Self { transport, host })
    }

    fn query<'a, Q>(&self, _: Q) -> FutureResponse<&'static str>
    where
        Q: Into<Query<'a>>,
    {
        let requesting = future::ok("moikka moi");

        FutureResponse(Box::new(requesting))
    }
}
