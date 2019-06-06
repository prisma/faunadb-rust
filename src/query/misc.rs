use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query![Abort, Class, Classes, Database, Databases];

/// This `Abort` function terminates the current transaction and augments the
/// returned error with the associated message.
///
/// Any modifications to data or schema in the aborted transaction will be
/// ignored, even if this modification took place before the abort function was
/// executed.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/abort)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Abort<'a> {
    abort: Expr<'a>,
}

impl<'a> Abort<'a> {
    pub fn new(msg: impl Into<Expr<'a>>) -> Self {
        Self { abort: msg.into() }
    }
}

/// The `Class` function returns a valid `Ref` for the given class name.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/class)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Class<'a> {
    class: Expr<'a>,
}

impl<'a> Class<'a> {
    pub fn find(name: impl Into<Expr<'a>>) -> Self {
        Self { class: name.into() }
    }
}

/// The `Classes` function when executed with `Paginate` returns an array of Refs
/// for all classes in the database specified.
///
/// If no database is provided, it returns an array of references to all classes
/// in the current database.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/classes)
#[derive(Serialize, Debug, Clone, Default, Deserialize)]
pub struct Classes<'a> {
    classes: Option<Expr<'a>>,
}

impl<'a> Classes<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            classes: Some(Expr::from(database)),
        }
    }
}

/// The `Database` function returns a valid `Ref` for the given database name.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/database)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Database<'a> {
    database: Expr<'a>,
}

impl<'a> Database<'a> {
    pub fn find(name: impl Into<Expr<'a>>) -> Self {
        Self {
            database: name.into(),
        }
    }
}

/// The `Databases` function when executed with `Paginate` returns an array of Refs
/// for sub-databases in the database specified. If no database is provided, it
/// returns an array of references to sub-databases in the current database.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/databases)
#[derive(Serialize, Debug, Clone, Default, Deserialize)]
pub struct Databases<'a> {
    databases: Option<Expr<'a>>,
}

impl<'a> Databases<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            databases: Some(Expr::from(database)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_abort() {
        let fun = Abort::new("BOOM");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "abort": "BOOM",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_class() {
        let fun = Class::find("housecats");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "class": "housecats",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_classes_all() {
        let fun = Classes::all();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "classes": null,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_classes_database() {
        let fun = Classes::from_database(Ref::database("cats"));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "classes": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "databases"
                        }
                    },
                    "id": "cats"
                }
            },
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_database() {
        let fun = Database::find("cats");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "database": "cats",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_databases_all() {
        let fun = Databases::all();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "databases": null,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_databases_atabase() {
        let fun = Databases::from_database(Ref::database("cats"));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "databases": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "databases"
                        }
                    },
                    "id": "cats"
                }
            },
        });

        assert_eq!(expected, serialized);
    }
}
