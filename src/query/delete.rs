use crate::expr::{Expr, Ref};

#[derive(Serialize, Debug, Clone)]
pub struct Delete<'a> {
    #[serde(flatten)]
    reference: Expr<'a>
}

impl<'a> Delete<'a> {
    pub fn instance(reference: Ref<'a>) -> Self {
        Delete {
            reference: Expr::from(reference),
        }
    }
}
