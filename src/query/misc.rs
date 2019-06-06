use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query![Abort, Class, Classes, Database, Databases, Function, Functions, Index, Indexes, NewId];

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

/// The `Function` function returns a valid `Ref` for the given function name.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/function)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Function<'a> {
    function: Expr<'a>,
}

impl<'a> Function<'a> {
    pub fn find(name: impl Into<Expr<'a>>) -> Self {
        Self {
            function: name.into(),
        }
    }
}

/// The `Functions` function when executed with `Paginate` returns an array of Refs
/// for all functions in the database specified.
///
/// If no database is provided, it returns an array of references to all functions
/// in the current database.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/functions)
#[derive(Serialize, Debug, Clone, Default, Deserialize)]
pub struct Functions<'a> {
    functions: Option<Expr<'a>>,
}

impl<'a> Functions<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            functions: Some(Expr::from(database)),
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
/// for sub-databases in the database specified.
///
/// If no database is provided, it returns an array of references to
/// sub-databases in the current database.
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

/// The `Index` function returns a valid `Ref` for the given index name.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/index)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Index<'a> {
    index: Expr<'a>,
}

impl<'a> Index<'a> {
    pub fn find(name: impl Into<Expr<'a>>) -> Self {
        Self { index: name.into() }
    }
}

/// The `Indexes` function when executed with `Paginate` returns an array of Refs
/// for indexes in the database specified.
///
/// If no database is provided, it returns an array of references to indexes in
/// the current database.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/databases)
#[derive(Serialize, Debug, Clone, Default, Deserialize)]
pub struct Indexes<'a> {
    indexes: Option<Expr<'a>>,
}

impl<'a> Indexes<'a> {
    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_database(database: Ref<'a>) -> Self {
        Self {
            indexes: Some(Expr::from(database)),
        }
    }
}

/// This `NewId` function produces a unique number.
///
/// This number is guaranteed to be unique across the entire cluster and once
/// generated is never generated a second time. This identifier is suitable for
/// constructing the id part of a reference.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/misc/newid)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct NewId<'a> {
    new_id: Expr<'a>,
}

impl<'a> NewId<'a> {
    pub fn new() -> Self {
        Self {
            new_id: Expr::null(),
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

    #[test]
    fn test_function() {
        let fun = Function::find("meow");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "function": "meow",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_functions_all() {
        let fun = Functions::all();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "functions": null,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_functions_database() {
        let fun = Functions::from_database(Ref::database("cats"));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "functions": {
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
    fn test_index() {
        let fun = Index::find("scratches");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "index": "scratches",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_indexes_all() {
        let fun = Indexes::all();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "indexes": null,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_indexes_database() {
        let fun = Indexes::from_database(Ref::database("cats"));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "indexes": {
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
    fn test_new_id() {
        let fun = NewId::new();

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "new_id": null,
        });

        assert_eq!(expected, serialized);
    }
}
