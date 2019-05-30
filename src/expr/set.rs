use crate::expr::{Expr, Ref};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Set<'a> {
    #[serde(rename = "match")]
    matching: Expr<'a>,
    terms: Expr<'a>,
}

impl<'a> Set<'a> {
    pub fn matching<E>(reference: Ref<'a>, terms: E) -> Self
    where
        E: Into<Expr<'a>>,
    {
        let matching = Expr::from(reference);
        let terms = terms.into();

        Self { matching, terms }
    }
}

impl<'a> fmt::Display for Set<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set(match={},terms={})", self.matching, self.terms)
    }
}
