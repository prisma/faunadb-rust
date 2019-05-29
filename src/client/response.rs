use crate::{
    error::Error,
    expr::{Expr, Object},
    serde::ts_microseconds,
};
use chrono::{DateTime, Utc};
use futures::{Future, Poll};

pub struct FutureResponse<T>(pub Box<Future<Item = T, Error = Error> + Send + 'static>);

impl<T> Future for FutureResponse<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Deserialize, Debug)]
pub enum Response {
    #[serde(rename = "resource")]
    Resource(Resource),
}

#[derive(Deserialize, Debug)]
pub struct Resource {
    #[serde(rename = "ref")]
    reference: Expr<'static>,
    #[serde(with = "ts_microseconds", rename = "ts")]
    timestamp: DateTime<Utc>,
    data: Object<'static>,
}
