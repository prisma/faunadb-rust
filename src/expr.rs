mod array;
mod object;
mod permission;
mod reference;
mod set;

use crate::{query::Query, serde::base64_bytes};
use chrono::{DateTime, NaiveDate, Utc};
use std::{borrow::Cow, fmt};

pub use array::{Array, Bytes};
pub use object::Object;
pub use permission::*;
pub use reference::Ref;
pub use set::Set;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// A simple expression with no annotation.
pub enum SimpleExpr<'a> {
    /// String data types store any letters, numbers, whitespaces, and/or
    /// symbols in a fixed order.
    String(Cow<'a, str>),
    /// Numbers are any real number which are bounded by double precision
    /// (64-bit), such as 3, -27, 3.1415. (Neither infinity nor NaN are
    /// allowed.)
    UInt(u64),
    /// Numbers are any real number which are bounded by double precision
    /// (64-bit), such as 3, -27, 3.1415. (Neither infinity nor NaN are
    /// allowed.)
    Int(i64),
    /// Numbers are any real number which are bounded by double precision
    /// (64-bit), such as 3, -27, 3.1415. (Neither infinity nor NaN are
    /// allowed.)
    Double(f64),
    /// Numbers are any real number which are bounded by double precision
    /// (64-bit), such as 3, -27, 3.1415. (Neither infinity nor NaN are
    /// allowed.)
    Float(f32),
    /// The boolean data type can only store "true" or "false" values. These can
    /// be directly compared for equality or inequality. They can also be
    /// compared to the Boolean literal values of `true` and `false`.
    Boolean(bool),
    /// An array is a data structure that contains a group of elements.
    /// Typically the elements of an array are of the same or related type. When
    /// an array is used in FQL it evaluates to its contents.
    Array(Array<'a>),
    /// For reading a value from a Fauna response. Due to a bug, Fauna sends
    /// objects back with no annotation.
    Object(Object<'a>),
    /// Null is a special marker used to indicate that a data value does not
    /// exist. It is a representation of missing information. A null value
    /// indicates a lack of a value. A lack of a value is not the same thing as
    /// a value of zero, in the same way that a lack of an answer is not the
    /// same thing as an answer of "no". Null is a value that can be directly
    /// compared for application programmer simplicity. This means that `Null == Null`
    /// returns `true`.
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A special expression with an annotation marker.
pub enum AnnotatedExpr<'a> {
    #[serde(rename = "@bytes", with = "base64_bytes")]
    /// Denotes a base64 encoded string representing a byte array.
    Bytes(Bytes<'a>),
    #[serde(rename = "@date")]
    /// Denotes a date, with no associated time zone.
    Date(NaiveDate),
    #[serde(rename = "@ref")]
    /// Denotes a resource ref. Refs may be extracted from instances, or
    /// constructed using the ref function.
    Ref(Ref<'a>),
    #[serde(rename = "@set")]
    /// Denotes a set identifier.
    Set(Box<Set<'a>>),
    #[serde(rename = "@ts")]
    /// Stores an instant in time expressed as a calendar date and time of
    /// day in UTC. A Timestamp can safely store nanoseconds precision, but be
    /// careful as many operating system clocks provide only microsecond
    /// precision. Timestamps may be inserted with offsets, but are converted to
    /// UTC; the offset component is lost. A time must be within the range
    /// `-999999999-01-01T00:00:00Z` - `9999-12-31T23:59:59.999999999Z`.
    Timestamp(DateTime<Utc>),
    #[serde(rename = "object")]
    /// Object values are a collection of key/value pairs. The keys must be
    /// strings and the values must be valid Fauna data types. The value
    /// expressions are evaluated sequentially in the order that they were
    /// specified, left to right. Objects evaluate to their contents:
    Object(Object<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// A representation of a FaunaDB Query Expression.
///
/// Expressions should be created using the `From`/`Into` traits.
///
/// See the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/types).
pub enum Expr<'a> {
    Annotated(AnnotatedExpr<'a>),
    Simple(SimpleExpr<'a>),
    #[serde(skip_deserializing)]
    Query(Box<Query<'a>>),
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Simple(SimpleExpr::String(s)) => write!(f, "\"{}\"", s),
            Expr::Simple(SimpleExpr::Double(d)) => write!(f, "{}", d),
            Expr::Simple(SimpleExpr::Float(flt)) => write!(f, "{}", flt),
            Expr::Simple(SimpleExpr::Int(i)) => write!(f, "{}", i),
            Expr::Simple(SimpleExpr::UInt(i)) => write!(f, "{}", i),
            Expr::Simple(SimpleExpr::Boolean(b)) => write!(f, "{}", b),
            Expr::Simple(SimpleExpr::Null) => write!(f, "null"),
            Expr::Simple(SimpleExpr::Array(v)) => {
                let exprs: Vec<String> = v.0.iter().map(|e| format!("{}", e)).collect();
                write!(f, "[{}]", exprs.join(","))
            }
            Expr::Simple(SimpleExpr::Object(o)) => write!(f, "{}", o),
            Expr::Annotated(AnnotatedExpr::Object(o)) => write!(f, "{}", o),
            Expr::Annotated(AnnotatedExpr::Bytes(b)) => write!(f, "{}", base64::encode(&b.0)),
            Expr::Annotated(AnnotatedExpr::Date(d)) => write!(f, "{}", d),
            Expr::Annotated(AnnotatedExpr::Ref(r)) => write!(f, "{}", r),
            Expr::Annotated(AnnotatedExpr::Set(s)) => write!(f, "{}", s),
            Expr::Annotated(AnnotatedExpr::Timestamp(ts)) => write!(f, "{}", ts),
            Expr::Query(query) => write!(f, "Query({:?})", query),
        }
    }
}

impl<'a> Expr<'a> {
    /// This hack is here for now for reusing the resulting `Expr` from FaunaDB.
    /// Due to a deficiency the resulting object will lose its annotation, and
    /// we must annotate it again for Fauna to accept the data.
    pub(crate) fn reuse(self) -> Self {
        match self {
            Expr::Simple(SimpleExpr::Object(o)) => {
                Expr::Annotated(AnnotatedExpr::Object(o.reuse()))
            }
            Expr::Annotated(AnnotatedExpr::Object(o)) => {
                Expr::Annotated(AnnotatedExpr::Object(o.reuse()))
            }
            Expr::Simple(SimpleExpr::Array(v)) => Expr::Simple(SimpleExpr::Array(v.reuse())),
            expr => expr,
        }
    }

    /// A helper to create a null expression.
    pub fn null() -> Self {
        Expr::Simple(SimpleExpr::Null)
    }
}

int_expr!(i8, i16, i32, i64);
uint_expr!(u8, u16, u32, u64);

impl<'a, T> From<Option<T>> for Expr<'a>
where
    T: Into<Expr<'a>>,
{
    fn from(t: Option<T>) -> Self {
        match t {
            Some(expr) => expr.into(),
            None => Expr::null(),
        }
    }
}

impl<'a> From<&'a str> for Expr<'a> {
    fn from(s: &'a str) -> Expr<'a> {
        Expr::Simple(SimpleExpr::String(Cow::from(s)))
    }
}

impl<'a> From<String> for Expr<'a> {
    fn from(s: String) -> Expr<'a> {
        Expr::Simple(SimpleExpr::String(Cow::from(s)))
    }
}

impl<'a> From<f64> for Expr<'a> {
    fn from(f: f64) -> Expr<'a> {
        Expr::Simple(SimpleExpr::Double(f))
    }
}

impl<'a> From<f32> for Expr<'a> {
    fn from(f: f32) -> Expr<'a> {
        Expr::Simple(SimpleExpr::Float(f))
    }
}

impl<'a> From<bool> for Expr<'a> {
    fn from(b: bool) -> Expr<'a> {
        Expr::Simple(SimpleExpr::Boolean(b))
    }
}

impl<'a> From<Array<'a>> for Expr<'a> {
    fn from(a: Array<'a>) -> Expr<'a> {
        Expr::Simple(SimpleExpr::Array(a))
    }
}

impl<'a, O> From<O> for Expr<'a>
where
    O: Into<Object<'a>>,
{
    fn from(o: O) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Object(o.into()))
    }
}

impl<'a> From<Bytes<'a>> for Expr<'a> {
    fn from(b: Bytes<'a>) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Bytes(b))
    }
}

impl<'a> From<Ref<'a>> for Expr<'a> {
    fn from(r: Ref<'a>) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Ref(r))
    }
}

impl<'a> From<NaiveDate> for Expr<'a> {
    fn from(d: NaiveDate) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Date(d))
    }
}

impl<'a> From<Set<'a>> for Expr<'a> {
    fn from(s: Set<'a>) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Set(Box::new(s)))
    }
}

impl<'a> From<DateTime<Utc>> for Expr<'a> {
    fn from(dt: DateTime<Utc>) -> Expr<'a> {
        Expr::Annotated(AnnotatedExpr::Timestamp(dt))
    }
}

impl<'a> From<Query<'a>> for Expr<'a> {
    fn from(q: Query<'a>) -> Expr<'a> {
        Expr::Query(Box::new(q))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{DateTime, NaiveDate, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_string_expr() {
        let expr = Expr::from("cat");
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("\"cat\"", serialized);
    }

    #[test]
    fn test_f64_expr() {
        let expr = Expr::from(4.12f64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4.12", serialized);
    }

    #[test]
    fn test_f32_expr() {
        let expr = Expr::from(4.12f32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4.12", serialized);
    }

    #[test]
    fn test_i8_expr() {
        let expr = Expr::from(4i8);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i16_expr() {
        let expr = Expr::from(4i16);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i32_expr() {
        let expr = Expr::from(4i32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_i64_expr() {
        let expr = Expr::from(4i64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u8_expr() {
        let expr = Expr::from(4u8);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u16_expr() {
        let expr = Expr::from(4u16);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u32_expr() {
        let expr = Expr::from(4u32);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_u64_expr() {
        let expr = Expr::from(4u64);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("4", serialized);
    }

    #[test]
    fn test_bytes_expr() {
        let expr = Expr::from(Bytes::from(vec![0x1, 0x2, 0x3, 0x4]));
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"@bytes\":\"AQIDBA==\"}", serialized)
    }

    #[test]
    fn test_bytes_deserialize() {
        match serde_json::from_str("{\"@bytes\":\"AQIDBA==\"}") {
            Ok(Expr::Annotated(AnnotatedExpr::Bytes(bytes))) => {
                assert_eq!(Bytes::from(vec![0x1, 0x2, 0x3, 0x4]), bytes)
            }
            expr => panic!("{:?} was not bytes", expr),
        }
    }

    #[test]
    fn test_date_expr() {
        let expr = Expr::from(NaiveDate::from_ymd(2001, 5, 31));
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"@date\":\"2001-05-31\"}", serialized)
    }

    #[test]
    fn test_ref_with_class_expr() {
        let mut refer = Ref::instance("foo");
        refer.set_class("test");

        let expr = Expr::from(refer);
        let serialized = serde_json::to_value(&expr).unwrap();

        let expected = json!({
            "@ref": {
                "class": {
                    "@ref": {
                        "class": {
                            "@ref": {
                                "id": "classes"
                            }
                        },
                        "id": "test"
                    }
                },
                "id": "foo"
            }
        });

        assert_eq!(expected, serialized)
    }

    #[test]
    fn test_bool_expr() {
        let expr = Expr::from(true);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("true", serialized)
    }

    #[test]
    fn test_null_expr() {
        let expr = Expr::null();
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("null", serialized)
    }

    #[test]
    fn test_simple_array_expr() {
        let array = Array::from(vec![Expr::from(1), Expr::from("test")]);
        let expr = Expr::from(array);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("[1,\"test\"]", serialized)
    }

    #[test]
    fn test_complex_array_expr() {
        let mut object = Object::default();
        object.insert("foo", "bar");
        object.insert("lol", false);

        let array = Array::from(vec![Expr::from(1), Expr::from(object)]);
        let expr = Expr::from(array);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!(
            "[1,{\"object\":{\"foo\":\"bar\",\"lol\":false}}]",
            serialized
        )
    }

    #[test]
    fn test_object_expr() {
        let mut object = Object::default();
        object.insert("foo", "bar");
        object.insert("lol", false);

        let expr = Expr::from(object);
        let serialized = serde_json::to_string(&expr).unwrap();

        assert_eq!("{\"object\":{\"foo\":\"bar\",\"lol\":false}}", serialized)
    }

    #[test]
    fn test_set_expr() {
        let set = Set::matching(Ref::index("cats_age"), 8);
        let expr = Expr::from(set);
        let serialized = serde_json::to_value(&expr).unwrap();

        let expected = json!({
            "@set": {
                "match": {
                    "@ref": {
                        "index": {
                            "@ref": {
                                "id": "indexes"
                            }
                        },
                        "id": "cats_age"
                    }
                },
                "terms": 8
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_set_timestamp_expr() {
        let dt_str = "2019-05-26T16:20:00Z";
        let dt = DateTime::parse_from_rfc3339(dt_str)
            .unwrap()
            .with_timezone(&Utc);

        let expr = Expr::from(dt);
        let serialized = serde_json::to_value(&expr).unwrap();

        let expected = json!({ "@ts": dt_str });

        assert_eq!(expected, serialized);
    }
}
