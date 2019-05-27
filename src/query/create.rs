use crate::expr::{Class, Expr, Ref};

#[derive(Debug, Serialize)]
pub struct Create<'a> {
    #[serde(rename = "@ref")]
    reference: Ref<'a>,
    #[serde(skip_serializing)]
    pub(crate) params: Expr<'a>,
}

impl<'a> Create<'a> {
    pub fn instance<E>(class: Class<'a>, params: E) -> Self
    where
        E: Into<Expr<'a>>,
    {
        let reference = Ref::class(class.id, class);
        let params = params.into();

        Self { reference, params }
    }
}
