use crate::{
    error::Error,
    expr::{Bytes, Number, Ref},
    serde::base64_bytes,
    FaunaResult,
};
use chrono::{DateTime, NaiveDate, Utc};
use futures::{Future, Poll};
use std::{collections::BTreeMap, convert::TryFrom};

pub struct FutureResponse<T>(pub Box<Future<Item = T, Error = Error> + Send + 'static>);

impl<T> Future for FutureResponse<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Response {
    pub resource: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SimpleValue {
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Null,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AnnotatedValue {
    #[serde(rename = "@ref")]
    Ref(Box<Ref<'static>>),
    #[serde(rename = "@query")]
    Query(Box<Value>),
    #[serde(rename = "@bytes", with = "base64_bytes")]
    Bytes(Bytes<'static>),
    #[serde(rename = "@date")]
    Date(NaiveDate),
    #[serde(rename = "@set")]
    Set(Box<Value>),
    #[serde(rename = "@ts")]
    Timestamp(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Annotated(AnnotatedValue),
    Simple(SimpleValue),
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::Simple(SimpleValue::String(s.to_string()))
    }
}

impl<T> From<T> for Value
where
    T: Into<Number>,
{
    fn from(t: T) -> Self {
        Value::Simple(SimpleValue::Number(t.into()))
    }
}

impl<V> From<Vec<V>> for Value
where
    V: Into<Value>,
{
    fn from(t: Vec<V>) -> Self {
        Value::Simple(SimpleValue::Array(t.into_iter().map(Into::into).collect()))
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(val: Value) -> FaunaResult<String> {
        match val {
            Value::Simple(SimpleValue::String(str)) => Ok(str),
            _ => Err(Error::ConversionError("Value is not a String")),
        }
    }
}

impl TryFrom<Value> for BTreeMap<String, Value> {
    type Error = Error;

    fn try_from(val: Value) -> FaunaResult<BTreeMap<String, Value>> {
        match val {
            Value::Simple(SimpleValue::Object(obj)) => Ok(obj),
            _ => Err(Error::ConversionError("Value is not an Object")),
        }
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = Error;

    fn try_from(val: Value) -> FaunaResult<Vec<Value>> {
        match val {
            Value::Simple(SimpleValue::Array(ary)) => Ok(ary),
            _ => Err(Error::ConversionError("Value is not an Array")),
        }
    }
}

impl Value {
    pub fn is_string(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::String(_)) => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Simple(SimpleValue::String(string)) => Some(string.as_str()),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(_)) => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> Option<Number> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => Some(*num),
            _ => None,
        }
    }

    pub fn is_u64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_u64(),
            _ => false,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_u64(),
            _ => None,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_i64(),
            _ => false,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_i64(),
            _ => None,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_f64(),
            _ => false,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_f64(),
            _ => None,
        }
    }

    pub fn is_f32(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_f32(),
            _ => false,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_f32(),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Boolean(_)) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Simple(SimpleValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Array(_)) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Simple(SimpleValue::Array(v)) => Some(v),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&Vec<Value>> {
        match self {
            Value::Simple(SimpleValue::Array(ref mut v)) => Some(v),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Object(_)) => true,
            _ => false,
        }
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Simple(SimpleValue::Object(obj)) => Some(obj),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match *self {
            Value::Simple(SimpleValue::Object(ref mut obj)) => Some(obj),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Null) => true,
            _ => false,
        }
    }

    pub fn is_reference(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Ref(_)) => true,
            _ => false,
        }
    }

    pub fn as_reference(&self) -> Option<&Ref<'static>> {
        match self {
            Value::Annotated(AnnotatedValue::Ref(reference)) => Some(&*reference),
            _ => None,
        }
    }

    pub fn is_query(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Query(_)) => true,
            _ => false,
        }
    }

    pub fn as_query(&self) -> Option<&Value> {
        match self {
            Value::Annotated(AnnotatedValue::Query(q)) => Some(&*q),
            _ => None,
        }
    }

    pub fn is_bytes(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Bytes(_)) => true,
            _ => false,
        }
    }

    pub fn as_bytes(&self) -> Option<&Bytes<'static>> {
        match self {
            Value::Annotated(AnnotatedValue::Bytes(byt)) => Some(byt),
            _ => None,
        }
    }

    pub fn is_date(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Date(_)) => true,
            _ => false,
        }
    }

    pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            Value::Annotated(AnnotatedValue::Date(dat)) => Some(*dat),
            _ => None,
        }
    }

    pub fn is_set(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Set(_)) => true,
            _ => false,
        }
    }

    pub fn as_set(&self) -> Option<&Value> {
        match self {
            Value::Annotated(AnnotatedValue::Set(set)) => Some(&*set),
            _ => None,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Timestamp(_)) => true,
            _ => false,
        }
    }

    pub fn as_timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Value::Annotated(AnnotatedValue::Timestamp(ts)) => Some(*ts),
            _ => None,
        }
    }
}
