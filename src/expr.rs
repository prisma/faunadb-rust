use serde::{ser::SerializeMap, ser::Serializer, Serialize};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Expr<'a> {
    String(&'a str),
    Double(f64),
    Float(f32),
    Int(i64),
    UInt(u64),
    Boolean(bool),
    Null,
    Object(Object<'a>),
}

impl<'a> Serialize for Expr<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Expr::String(s) => serializer.serialize_str(*s),
            Expr::Double(d) => serializer.serialize_f64(*d),
            Expr::Float(f) => serializer.serialize_f32(*f),
            Expr::Int(i) => serializer.serialize_i64(*i),
            Expr::UInt(i) => serializer.serialize_u64(*i),
            Expr::Boolean(b) => serializer.serialize_bool(*b),
            Expr::Null => serializer.serialize_none(),
            Expr::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                map.serialize_entry("object", &obj)?;
                map.end()
            }
        }
    }
}

impl<'a> From<&'a str> for Expr<'a> {
    fn from(s: &'a str) -> Expr<'a> {
        Expr::String(s)
    }
}

impl<'a> From<f64> for Expr<'a> {
    fn from(f: f64) -> Expr<'a> {
        Expr::Double(f)
    }
}

impl<'a> From<f32> for Expr<'a> {
    fn from(f: f32) -> Expr<'a> {
        Expr::Float(f)
    }
}

impl<'a> From<i8> for Expr<'a> {
    fn from(i: i8) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i16> for Expr<'a> {
    fn from(i: i16) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i32> for Expr<'a> {
    fn from(i: i32) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<i64> for Expr<'a> {
    fn from(i: i64) -> Expr<'a> {
        Expr::Int(i)
    }
}

impl<'a> From<isize> for Expr<'a> {
    fn from(i: isize) -> Expr<'a> {
        Expr::Int(i as i64)
    }
}

impl<'a> From<u8> for Expr<'a> {
    fn from(u: u8) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u16> for Expr<'a> {
    fn from(u: u16) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u32> for Expr<'a> {
    fn from(u: u32) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<u64> for Expr<'a> {
    fn from(u: u64) -> Expr<'a> {
        Expr::UInt(u)
    }
}

impl<'a> From<usize> for Expr<'a> {
    fn from(u: usize) -> Expr<'a> {
        Expr::UInt(u as u64)
    }
}

impl<'a> From<bool> for Expr<'a> {
    fn from(b: bool) -> Expr<'a> {
        Expr::Boolean(b)
    }
}

impl<'a> From<Object<'a>> for Expr<'a> {
    fn from(o: Object<'a>) -> Expr<'a> {
        Expr::Object(o)
    }
}

#[derive(Debug, Serialize)]
pub struct Object<'a> {
    #[serde(flatten)]
    data: BTreeMap<&'a str, Expr<'a>>,
}

impl<'a> From<BTreeMap<&'a str, Expr<'a>>> for Object<'a> {
    fn from(data: BTreeMap<&'a str, Expr<'a>>) -> Self {
        Object { data }
    }
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::default(),
        }
    }

    pub fn insert<E>(&mut self, key: &'a str, val: E) -> &mut Self
    where
        E: Into<Expr<'a>>,
    {
        self.data.insert(key, val.into());
        self
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json;

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
}
