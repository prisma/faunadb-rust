mod create;
mod create_database;
mod create_class;
mod get;

pub use create::*;
pub use create_class::*;
pub use create_database::*;
pub use get::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug)]
pub enum Query<'a> {
    Create(Create<'a>),
    CreateClass(CreateClass<'a>),
    CreateDatabase(CreateDatabase<'a>),
    Get(Get<'a>),
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
            Query::Get(get) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("get", &get)?;

                if let Some(ref ts) = get.timestamp {
                    let microseconds = (ts.timestamp_nanos() / 1000) as i64;
                    map.serialize_entry("ts", &microseconds)?;
                }

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
        Query::CreateClass(create)
    }
}

impl<'a> From<Get<'a>> for Query<'a> {
    fn from(get: Get<'a>) -> Self {
        Query::Get(get)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_instance() {
        let mut params = Object::new();
        params.insert("test_field", "test_value");

        let mut data = Object::new();
        data.insert("data", params);

        let query = Query::from(Create::instance(Ref::class("test"), data));
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
}
