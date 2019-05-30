use crate::{
    error::Error,
    expr::{Expr, Object},
    serde::ts_microseconds,
};
use chrono::{DateTime, Utc};
use futures::{Future, Poll};
use std::fmt;

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

impl Response {
    pub fn clone_data(&self) -> Option<Object<'static>> {
        match self {
            Response::Resource(Resource::Instance(ref inst)) => Some(inst.data.clone().reuse()),
            _ => None
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Instance {
    #[serde(rename = "ref")]
    reference: Expr<'static>,
    #[serde(with = "ts_microseconds", rename = "ts")]
    timestamp: DateTime<Utc>,
    data: Object<'static>,
}

#[derive(Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "ref")]
    reference: Expr<'static>,
    #[serde(with = "ts_microseconds", rename = "ts")]
    timestamp: DateTime<Utc>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    history_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_days: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Resource {
    Instance(Instance),
    Class(Class)
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Response::Resource(Resource::Instance(res)) => write!(
                f,
                "Instance(ref={},data={},ts={})",
                res.reference,
                res.data,
                res.timestamp,
            ),
            Response::Resource(Resource::Class(res)) => write!(
                f,
                "Class(ref={},name={},history={:?},ttl={:?},ts={})",
                res.reference,
                res.name,
                res.history_days,
                res.ttl_days,
                res.timestamp,
            ),
        }
    }
}
