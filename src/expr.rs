mod object;
mod reference;
mod set;
mod array;
mod permission;

use crate::serde::base64_bytes;
use chrono::{DateTime, NaiveDate, Utc};
use std::{borrow::Cow, fmt};

pub use object::Object;
pub use reference::Ref;
pub use set::Set;
pub use array::{Array, Bytes};
pub use permission::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleExpr<'a> {
    String(Cow<'a, str>),
    UInt(u64),
    Int(i64),
    Double(f64),
    Float(f32),
    Boolean(bool),
    Array(Array<'a>),
    Object(Object<'a>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotatedExpr<'a> {
    #[serde(rename = "@bytes", with = "base64_bytes")]
    Bytes(Bytes<'a>),
    #[serde(rename = "@date")]
    Date(NaiveDate),
    #[serde(rename = "@ref")]
    Ref(Ref<'a>),
    #[serde(rename = "@set")]
    Set(Box<Set<'a>>),
    #[serde(rename = "@ts")]
    Timestamp(DateTime<Utc>),
    #[serde(rename = "object")]
    Object(Object<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expr<'a> {
    Annotated(AnnotatedExpr<'a>),
    Simple(SimpleExpr<'a>),
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
        }
    }
}

impl<'a> Expr<'a> {
    /// Use this hack for now when reusing resulting `Expr` from FaunaDB. Due to
    /// a deficiency the resulting object will lose its annotation, and we must
    /// annotate it again for Fauna to accept the data.
    pub fn reuse(self) -> Self {
        match self {
            Expr::Simple(SimpleExpr::Object(o)) => {
                Expr::Annotated(AnnotatedExpr::Object(o.reuse()))
            },
            Expr::Annotated(AnnotatedExpr::Object(o)) => {
                Expr::Annotated(AnnotatedExpr::Object(o.reuse()))
            },
            Expr::Simple(SimpleExpr::Array(v)) => {
                Expr::Simple(SimpleExpr::Array(v.reuse()))
            },
            expr => expr,
        }
    }

    pub fn null() -> Self {
        Expr::Simple(SimpleExpr::Null)
    }
}

macro_rules! int_expr {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind> for Expr<'a> {
                fn from(i: $kind) -> Expr<'a> {
                    Expr::Simple(SimpleExpr::Int(i as i64))
                }
            }
        )*
    );
}

macro_rules! uint_expr {
    ($($kind:ident),*) => (
        $(
            impl<'a> From<$kind> for Expr<'a> {
                fn from(u: $kind) -> Expr<'a> {
                    Expr::Simple(SimpleExpr::UInt(u as u64))
                }
            }
        )*
    );
}

int_expr!(i8, i16, i32, i64, isize);
uint_expr!(u8, u16, u32, u64, usize);

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
    fn test_isize_expr() {
        let expr = Expr::from(4isize);
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
    fn test_usize_expr() {
        let expr = Expr::from(4usize);
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
            Ok(Expr::Annotated(AnnotatedExpr::Bytes(bytes))) =>
                assert_eq!(Bytes::from(vec![0x1, 0x2, 0x3, 0x4]), bytes),
            expr => panic!("{:?} was not bytes", expr)
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
        let mut object = Object::new();
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
        let mut object = Object::new();
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
