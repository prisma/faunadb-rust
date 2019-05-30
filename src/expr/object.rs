use crate::expr::Expr;
use std::{borrow::Cow, collections::BTreeMap, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl<'a> fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pairs: Vec<String> = self
            .data
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();

        write!(f, "{{{}}}", pairs.join(","))
    }
}
