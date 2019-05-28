use crate::expr::Expr;
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Debug, Serialize, Clone)]
pub struct Object<'a> {
    #[serde(flatten)]
    data: BTreeMap<Cow<'a, str>, Expr<'a>>,
}

impl<'a> From<BTreeMap<Cow<'a, str>, Expr<'a>>> for Object<'a> {
    fn from(data: BTreeMap<Cow<'a, str>, Expr<'a>>) -> Self {
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
        self.data.insert(Cow::from(key), val.into());
        self
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
