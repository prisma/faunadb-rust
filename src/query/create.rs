use crate::expr::{Expr, Ref};

#[derive(Debug, Serialize)]
pub struct Create<'a> {
    #[serde(flatten)]
    reference: Expr<'a>,
    #[serde(skip_serializing)]
    pub(crate) params: Expr<'a>,
}

impl<'a> Create<'a> {
    pub fn instance<E>(reference: Ref<'a>, params: E) -> Self
    where
        E: Into<Expr<'a>>,
    {
        let params = params.into();
        let reference = Expr::from(reference);

        Self { reference, params }
    }
}
