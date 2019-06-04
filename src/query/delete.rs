use crate::expr::{Expr, Ref};

#[derive(Serialize, Debug, Clone)]
pub struct Delete<'a> {
    delete: Expr<'a>,
}

impl<'a> Delete<'a> {
    pub fn instance(reference: Ref<'a>) -> Self {
        Delete {
            delete: Expr::from(reference),
        }
    }
}
