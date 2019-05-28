mod create;
mod get;

pub use create::*;
pub use get::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug)]
pub enum WriteQuery<'a> {
    Create(Create<'a>),
}

impl<'a> Serialize for WriteQuery<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WriteQuery::Create(create) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("create", &create)?;
                map.serialize_entry("params", &create.params)?;
                map.end()
            }
        }
    }
}

impl<'a> From<Create<'a>> for WriteQuery<'a> {
    fn from(create: Create<'a>) -> Self {
        WriteQuery::Create(create)
    }
}

#[derive(Debug)]
pub enum ReadQuery<'a> {
    Get(Get<'a>),
}

impl<'a> Serialize for ReadQuery<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ReadQuery::Get(get) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("get", &get)?;

                if let Some(ref ts) = get.timestamp {
                    map.serialize_entry("ts", ts)?;
                }

                map.end()
            }
        }
    }
}

impl<'a> From<Get<'a>> for ReadQuery<'a> {
    fn from(get: Get<'a>) -> Self {
        ReadQuery::Get(get)
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

        let query = WriteQuery::from(Create::instance(Ref::class("test"), data));
        let serialized = serde_json::to_string(&query).unwrap();

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
                "@ref": "classes/test"
            }
        });

        let expected_str = serde_json::to_string(&expected).unwrap();

        assert_eq!(expected_str, serialized);
    }
}
