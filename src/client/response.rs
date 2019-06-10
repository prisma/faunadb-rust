use crate::{
    error::Error,
    expr::{Bytes, Ref},
    serde::base64_bytes,
};
use chrono::{DateTime, NaiveDate, Utc};
use futures::{Future, Poll};
use std::collections::BTreeMap;

pub struct FutureResponse<T>(pub Box<Future<Item = T, Error = Error> + Send + 'static>);

impl<T> Future for FutureResponse<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SimpleValue {
    String(String),
    UInt(u64),
    Int(i64),
    Double(f64),
    Boolean(bool),
    Array(Vec<SimpleValue>),
    Object(BTreeMap<String, Resource>),
    Null,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnnotatedValue {
    #[serde(rename = "@ref")]
    Ref(Box<Ref<'static>>),
    #[serde(rename = "@query")]
    Query(Box<Resource>),
    #[serde(rename = "@bytes", with = "base64_bytes")]
    Bytes(Bytes<'static>),
    #[serde(rename = "@date")]
    Date(NaiveDate),
    #[serde(rename = "@set")]
    Set(Box<Resource>),
    #[serde(rename = "@ts")]
    Timestamp(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Resource {
    Annotated(AnnotatedValue),
    Simple(SimpleValue),
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub resource: Resource,
}
