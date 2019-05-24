use crate::expr::{Expr, Ref};

#[derive(Debug, Serialize)]
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
