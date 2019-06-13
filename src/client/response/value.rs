use super::ValueIndex;
use crate::{
    error::Error,
    expr::{Bytes, Number, Ref},
    serde::base64_bytes,
    FaunaResult,
};
use chrono::{DateTime, NaiveDate, Utc};
use std::{collections::BTreeMap, convert::TryFrom};

/// Represents any value returned from Fauna.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/types)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Value {
    /// A value with an annotation for its type definition.
    Annotated(AnnotatedValue),
    /// A value with a direct mapping to the types supported in JSON.
    Simple(SimpleValue),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SimpleValue {
    /// String data types store any letters, numbers, whitespaces, and/or symbols in a fixed order.
    String(String),
    /// Numbers are any real number which are bounded by double precision (64-bit).
    Number(Number),
    /// The boolean data type can only store "true" or "false" values. These can
    /// be directly compared for equality or inequality.
    Boolean(bool),
    /// An array is a data structure that contains a group of elements.
    /// Typically the elements of an array are of the same or related type.
    Array(Vec<Value>),
    /// Object values are a collection of key/value pairs.
    Object(BTreeMap<String, Value>),
    /// Null is a special marker used to indicate that a data value does not
    /// exist. It is a representation of missing information. A null value
    /// indicates a lack of a value. A lack of a value is not the same thing as
    /// a value of zero, in the same way that a lack of an answer is not the
    /// same thing as an answer of "no".
    Null,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AnnotatedValue {
    /// Denotes a resource ref. Refs may be extracted from instances, or
    /// constructed using the ref function.
    #[serde(rename = "@ref")]
    Ref(Ref<'static>),
    /// Denotes a query expression object.
    #[serde(rename = "@query")]
    Query(Box<Value>),
    /// Denotes a base64 encoded string representing a byte array. Decoded to
    /// bytes when deserialized.
    #[serde(rename = "@bytes", with = "base64_bytes")]
    Bytes(Bytes<'static>),
    /// Denotes a date, with no associated time zone.
    #[serde(rename = "@date")]
    Date(NaiveDate),
    /// Denotes a set identifier.
    #[serde(rename = "@set")]
    Set(Box<Value>),
    /// Stores an instant in time expressed as a calendar date and time of day
    /// in UTC.
    #[serde(rename = "@ts")]
    Timestamp(DateTime<Utc>),
}

impl Default for Value {
    fn default() -> Self {
        Value::null()
    }
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

impl From<Ref<'static>> for Value {
    fn from(t: Ref<'static>) -> Self {
        Value::Annotated(AnnotatedValue::Ref(t))
    }
}

impl From<NaiveDate> for Value {
    fn from(t: NaiveDate) -> Self {
        Value::Annotated(AnnotatedValue::Date(t))
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(t: DateTime<Utc>) -> Self {
        Value::Annotated(AnnotatedValue::Timestamp(t))
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

impl<S, V> From<BTreeMap<S, V>> for Value
where
    S: Into<String>,
    V: Into<Value>,
{
    fn from(map: BTreeMap<S, V>) -> Self {
        let obj = map.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        Value::Simple(SimpleValue::Object(obj))
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
    /// A helper to get a `Null` value.
    pub const fn null() -> Value {
        Value::Simple(SimpleValue::Null)
    }

    /// Index into a Fauna `Array` or `Object`. A string index can be used to
    /// access a value in an `Object`, and a usize index can be used to access
    /// an element of an `Array`.
    ///
    /// Returns `None` if the type of `self` does not match the type of the index
    /// or the given key does not exist in the map or the given index is not
    /// within the bounds of the array.
    ///
    /// ```
    /// # use faunadb::prelude::*;
    /// # use std::collections::BTreeMap;
    /// #
    /// let mut obj = BTreeMap::new();
    /// obj.insert("foo", "bar");
    ///
    /// let value = Value::from(vec![obj]);
    /// assert_eq!(Some("bar"), value[0]["foo"].as_str());
    /// ```
    pub fn get<I: ValueIndex>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    /// Mutably index into a Fauna `Array` or `Object`. A string index can be
    /// used to access a value in an `Object`, and a usize index can be used to
    /// access an element of an `Array`.
    ///
    /// Returns `None` if the type of `self` does not match the type of the index
    /// or the given key does not exist in the map or the given index is not
    /// within the bounds of the array.
    ///
    /// ```
    /// # use faunadb::prelude::*;
    /// # use std::collections::BTreeMap;
    /// #
    /// let mut obj = BTreeMap::new();
    /// obj.insert("cat", "purr");
    ///
    /// let mut obj_value = Value::from(obj);
    /// *obj_value.get_mut("cat").unwrap() = Value::from("meow");
    ///
    /// let mut ary_value = Value::from(vec!["meow"]);
    /// *ary_value.get_mut(0).unwrap() = Value::from("purr");
    /// ```
    pub fn get_mut<I: ValueIndex>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    /// `true` if the `Value` is a `String`.
    pub fn is_string(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::String(_)) => true,
            _ => false,
        }
    }

    /// Returns a &str if the value is a `String`, otherwise `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Simple(SimpleValue::String(string)) => Some(string.as_str()),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `Number`.
    pub fn is_number(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(_)) => true,
            _ => false,
        }
    }

    /// Returns a `Number` for number values, otherwise `None`.
    pub fn as_number(&self) -> Option<Number> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => Some(*num),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `u64`.
    pub fn is_u64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_u64(),
            _ => false,
        }
    }

    /// Returns a `u64` for `u64` values, otherwise `None`.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_u64(),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `i64`.
    pub fn is_i64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_i64(),
            _ => false,
        }
    }

    /// Returns a `i64` for `i64` values, otherwise `None`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_i64(),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `f64`.
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_f64(),
            _ => false,
        }
    }

    /// Returns a `f64` for `f64` values, otherwise `None`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_f64(),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `f32`.
    pub fn is_f32(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.is_f32(),
            _ => false,
        }
    }

    /// Returns a `f32` for `f32` values, otherwise `None`.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Value::Simple(SimpleValue::Number(num)) => num.as_f32(),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `bool`.
    pub fn is_bool(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Boolean(_)) => true,
            _ => false,
        }
    }

    /// Returns a `bool` for `bool` values, otherwise `None`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Simple(SimpleValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    /// `true` if the `Value` is an `Array`.
    pub fn is_array(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Array(_)) => true,
            _ => false,
        }
    }

    /// Returns an `Array` for `Array` values, otherwise `None`.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Simple(SimpleValue::Array(v)) => Some(v),
            _ => None,
        }
    }

    /// Returns a mutable `Array` for `Array` values, otherwise `None`.
    pub fn as_array_mut(&mut self) -> Option<&Vec<Value>> {
        match self {
            Value::Simple(SimpleValue::Array(ref mut v)) => Some(v),
            _ => None,
        }
    }

    /// `true` if the `Value` is an `Object`.
    pub fn is_object(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Object(_)) => true,
            _ => false,
        }
    }

    /// Returns an `Object` for `Object` values, otherwise `None`.
    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Simple(SimpleValue::Object(obj)) => Some(obj),
            _ => None,
        }
    }

    /// Returns a mutable `Object` for `Object` values, otherwise `None`.
    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match *self {
            Value::Simple(SimpleValue::Object(ref mut obj)) => Some(obj),
            _ => None,
        }
    }

    /// `true` if the `Value` is `Null`.
    pub fn is_null(&self) -> bool {
        match self {
            Value::Simple(SimpleValue::Null) => true,
            _ => false,
        }
    }

    /// `true` if the `Value` is a `Ref`.
    pub fn is_reference(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Ref(_)) => true,
            _ => false,
        }
    }

    /// Returns a `Ref` for `Ref` values, otherwise `None`.
    pub fn as_reference(&self) -> Option<&Ref<'static>> {
        match self {
            Value::Annotated(AnnotatedValue::Ref(reference)) => Some(&*reference),
            _ => None,
        }
    }

    /// Finds a nearest `Ref` if found and if taken from an `Object`, otherwise
    /// `None`.
    pub fn get_reference(&self) -> Option<&Ref<'static>> {
        self["ref"].as_reference()
    }

    /// `true` if the `Value` is a `Query`.
    pub fn is_query(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Query(_)) => true,
            _ => false,
        }
    }

    /// Returns a `Query` for `Query` values, otherwise `None`.
    pub fn as_query(&self) -> Option<&Value> {
        match self {
            Value::Annotated(AnnotatedValue::Query(q)) => Some(&*q),
            _ => None,
        }
    }

    /// `true` if the `Value` is a set of `Bytes`.
    pub fn is_bytes(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Bytes(_)) => true,
            _ => false,
        }
    }

    /// Returns `Bytes` for sets of `Bytes`, otherwise `None`.
    pub fn as_bytes(&self) -> Option<&Bytes<'static>> {
        match self {
            Value::Annotated(AnnotatedValue::Bytes(byt)) => Some(byt),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `Date`.
    pub fn is_date(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Date(_)) => true,
            _ => false,
        }
    }

    /// Returns a `NaiveDate` for `Date` values, otherwise `None`.
    pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            Value::Annotated(AnnotatedValue::Date(dat)) => Some(*dat),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `Set`.
    pub fn is_set(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Set(_)) => true,
            _ => false,
        }
    }

    /// Returns a `Set` for `Set` values, otherwise `None`.
    pub fn as_set(&self) -> Option<&Value> {
        match self {
            Value::Annotated(AnnotatedValue::Set(set)) => Some(&*set),
            _ => None,
        }
    }

    /// `true` if the `Value` is a `Timestamp`.
    pub fn is_timestamp(&self) -> bool {
        match self {
            Value::Annotated(AnnotatedValue::Timestamp(_)) => true,
            _ => false,
        }
    }

    /// Returns a `DateTime` for `Timestamp` values, otherwise `None`.
    pub fn as_timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Value::Annotated(AnnotatedValue::Timestamp(ts)) => Some(*ts),
            _ => None,
        }
    }
}
