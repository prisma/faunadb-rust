use crate::{
    expr::{Expr, Object, Ref},
    query::Query,
};
use chrono::{DateTime, Utc};

mod create;
mod create_class;
mod create_database;
mod create_function;
mod create_index;
mod create_key;
mod insert;
mod update;

pub use create::*;
pub use create_class::*;
pub use create_database::*;
pub use create_function::*;
pub use create_index::*;
pub use create_key::*;
pub use insert::*;
pub use update::*;

query![Delete, Remove, Replace];

#[derive(Serialize, Debug, Clone, Deserialize, Copy)]
pub enum Action {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "update")]
    Update,
}

/// The delete function removes an object. Some of the common objects to delete
/// are instances, classes, indexes and databases.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/delete)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Delete<'a> {
    delete: Expr<'a>,
}

impl<'a> Delete<'a> {
    pub fn new(reference: Ref<'a>) -> Self {
        Delete {
            delete: Expr::from(reference),
        }
    }
}

/// The `Remove` function deletes an event from an instance’s history.
///
/// The reference must refer to an instance of a user-defined class.
///
/// Outstanding references result in an "invalid argument" error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/remove)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Remove<'a> {
    remove: Expr<'a>,
    #[serde(rename = "ts")]
    timestamp: Expr<'a>,
    action: Action,
}

impl<'a> Remove<'a> {
    pub fn new(reference: Ref<'a>, timestamp: DateTime<Utc>, action: Action) -> Self {
        Self {
            remove: Expr::from(reference),
            timestamp: Expr::from(timestamp),
            action,
        }
    }
}

/// The `Replace` operation substitutes the user data pointed to by the reference
/// with the data contained in the `param_object`. Values not specified in the
/// `param_object` are removed.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/replace)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Replace<'a> {
    replace: Expr<'a>,
    params: Expr<'a>,
}

impl<'a> Replace<'a> {
    pub fn new(reference: Ref<'a>, params: Object<'a>) -> Self {
        Self {
            replace: Expr::from(reference),
            params: Expr::from(params),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_delete() {
        let delete = Delete::new(Ref::instance("musti"));
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

    #[test]
    fn test_remove() {
        let fun = Remove::new(
            Ref::instance("naukio"),
            Utc.timestamp(60, 0),
            Action::Create,
        );

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "remove": {
                "@ref": {
                    "id": "naukio"
                }
            },
            "ts": {
                "@ts": "1970-01-01T00:01:00Z"
            },
            "action": "create",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_replace() {
        let mut data = Object::default();
        data.insert("pawpaw", "meowmeow");

        let fun = Replace::new(Ref::instance("musti"), data);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "replace": {
                "@ref": {
                    "id": "musti"
                }
            },
            "params": {
                "object": {
                    "pawpaw": "meowmeow"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
