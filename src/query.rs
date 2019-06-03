mod create;
mod create_class;
mod create_database;
mod create_index;
mod delete;
mod do_many;
mod get;

pub use create::*;
pub use create_class::*;
pub use create_database::*;
pub use create_index::*;
pub use delete::*;
pub use do_many::*;
pub use get::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug)]
pub enum Query<'a> {
    Create(Create<'a>),
    CreateClass(Box<CreateClass<'a>>),
    CreateDatabase(CreateDatabase<'a>),
    CreateIndex(Box<CreateIndex<'a>>),
    Delete(Delete<'a>),
    Get(Get<'a>),
    Do(Do<'a>),
}

impl<'a> Serialize for Query<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Query::Create(create) => {
                let mut map = serializer.serialize_map(Some(2))?;

                map.serialize_entry("create", &create)?;
                map.serialize_entry("params", &create.params)?;
                map.end()
            }
            Query::CreateClass(create_class) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("create_class", &create_class)?;
                map.end()
            }
            Query::CreateDatabase(create_database) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("create_database", &create_database)?;
                map.end()
            }
            Query::CreateIndex(create_index) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("create_index", &create_index)?;
                map.end()
            }
            Query::Get(get) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("get", &get)?;

                if let Some(ref ts) = get.timestamp {
                    map.serialize_entry("ts", &ts)?;
                }

                map.end()
            }
            Query::Delete(delete) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("delete", &delete)?;
                map.end()
            }
            Query::Do(do_many) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("do", &do_many)?;
                map.end()
            }
        }
    }
}

impl<'a> From<Create<'a>> for Query<'a> {
    fn from(create: Create<'a>) -> Self {
        Query::Create(create)
    }
}

impl<'a> From<CreateClass<'a>> for Query<'a> {
    fn from(create: CreateClass<'a>) -> Self {
        Query::CreateClass(Box::new(create))
    }
}

impl<'a> From<CreateDatabase<'a>> for Query<'a> {
    fn from(create: CreateDatabase<'a>) -> Self {
        Query::CreateDatabase(create)
    }
}

impl<'a> From<CreateIndex<'a>> for Query<'a> {
    fn from(create: CreateIndex<'a>) -> Self {
        Query::CreateIndex(Box::new(create))
    }
}

impl<'a> From<Get<'a>> for Query<'a> {
    fn from(get: Get<'a>) -> Self {
        Query::Get(get)
    }
}

impl<'a> From<Delete<'a>> for Query<'a> {
    fn from(delete: Delete<'a>) -> Self {
        Query::Delete(delete)
    }
}

impl<'a> From<Do<'a>> for Query<'a> {
    fn from(do_many: Do<'a>) -> Self {
        Query::Do(do_many)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_create() {
        let mut obj = Object::default();
        obj.insert("test_field", "test_value");

        let params = InstanceParams::new(obj);

        let query = Query::from(Create::new(Ref::class("test"), params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "params": {
                "object": {
                    "data": {
                        "object": {
                            "test_field": "test_value"
                        }
                    }
                }
            },
            "create": {
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "classes"
                        }
                    },
                    "id": "test",
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_class() {
        let mut permission = ClassPermission::default();
        permission.read(Level::public());

        let mut params = ClassParams::new("test");
        params.history_days(10);
        params.permissions(permission);

        let query = Query::from(CreateClass::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_class": {
                "object": {
                    "history_days": 10,
                    "name": "test",
                    "permissions": { "object": { "read": "public" } },
                    "ttl_days": null,
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_index() {
        let mut permission = IndexPermission::default();
        permission.read(Level::public());

        let mut params = IndexParams::new("meows", Ref::class("cats"));
        params.permissions(permission);

        let age_term = Term::field(vec!["data", "age"]);
        let name_term = Term::binding("cats_name");

        params.terms(vec![age_term, name_term]);

        let name_value = Value::field(vec!["data", "name"]);

        let mut age_value = Value::binding("cats_age");
        age_value.reverse();

        params.values(vec![age_value, name_value]);

        let query = Query::from(CreateIndex::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_index": {
                "object": {
                    "active": false,
                    "name": "meows",
                    "permissions": {
                        "object": {
                            "read": "public",
                        }
                    },
                    "serialized": false,
                    "source": {
                        "@ref": {
                            "class": {
                                "@ref": {
                                    "id": "classes",
                                },
                            },
                            "id": "cats",
                        },
                    },
                    "terms": [
                        {
                            "object": {
                                "field": ["data", "age"],
                            }
                        },
                        {
                            "object": {
                                "binding": "cats_name",
                            }
                        },
                    ],
                    "unique": false,
                    "values": [
                        {
                            "object": {
                                "binding": "cats_age",
                                "reverse": true,
                            }
                        },
                        {
                            "object": {
                                "field": ["data", "name"],
                                "reverse": false,
                            }
                        },
                    ]
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_create_database() {
        let mut params = DatabaseParams::new("test");
        params.priority(10).unwrap();

        let query = Query::from(CreateDatabase::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_database": {
                "object": {
                    "name": "test",
                    "api_version": 2.0,
                    "priority": 10,
                }
            }
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_get() {
        let mut get = Get::instance(Ref::instance("musti"));
        get.timestamp(Utc.timestamp(60, 0));

        let query = Query::from(get);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "get": {
                "@ref": {
                    "id": "musti"
                }
            },
            "ts": {
                "@ts": Utc.timestamp(60, 0)
            }
        });

        assert_eq!(expected, serialized);
    }

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

    #[test]
    fn test_do() {
        let mut do_many = Do::new(Get::instance(Ref::instance("musti")));
        do_many.push(Delete::instance(Ref::instance("musti")));

        let query = Query::from(do_many);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "do": [
                {"get": {"@ref": {"id": "musti"}}},
                {"delete": {"@ref": {"id": "musti"}}},
            ]
        });

        assert_eq!(expected, serialized);
    }
}
