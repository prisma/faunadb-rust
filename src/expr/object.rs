use crate::expr::Expr;
use std::{borrow::Cow, collections::BTreeMap, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object<'a>(pub(crate) BTreeMap<Cow<'a, str>, Expr<'a>>);

impl<'a> From<BTreeMap<Cow<'a, str>, Expr<'a>>> for Object<'a> {
    fn from(data: BTreeMap<Cow<'a, str>, Expr<'a>>) -> Self {
        Object(data)
    }
}

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Self(BTreeMap::default())
    }

    pub fn insert<E>(&mut self, key: &'a str, val: E) -> &mut Self
    where
        E: Into<Expr<'a>>,
    {
        self.0.insert(Cow::from(key), val.into());
        self
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn reuse(self) -> Self {
        let reused = self.0.into_iter().map(|(k, v)| (k, v.reuse())).collect();
        Object(reused)
    }
}

impl<'a> fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pairs: Vec<String> = self
            .0
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();

        write!(f, "{{{}}}", pairs.join(","))
    }
}
