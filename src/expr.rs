use serde::{ser::SerializeMap, ser::Serializer, Serialize};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Expr<'a> {
    String(&'a str),
    Number(f64),
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
            Expr::Number(f) => serializer.serialize_f64(*f),
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
        Expr::Number(f)
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
