use crate::{
    expr::{Expr, Ref},
    query::Query,
};

mod create;
mod create_class;
mod create_database;
mod create_index;

pub use create::*;
pub use create_class::*;
pub use create_database::*;
pub use create_index::*;

query!(Delete);

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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_delete() {
        let delete = Delete::instance(Ref::instance("musti"));
        let query = Query::from(delete);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "delete": {
                "@ref": {
                    "id": "musti"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
