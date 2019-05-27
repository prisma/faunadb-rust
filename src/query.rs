mod create;

pub use create::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug)]
pub enum Query<'a> {
    Create(Create<'a>),
}

impl<'a> Serialize for Query<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Query::Create(create) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("create", &create)?;
                map.serialize_entry("params", &create.params)?;
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

        let query = Query::from(Create::instance(Class::new("test"), data));
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
                "@ref": {
                    "class": {
                        "@ref": {
                            "id": "classes"
                        }
                    },
                    "id": "test"
                }
            }
        });

        let expected_str = serde_json::to_string(&expected).unwrap();

        assert_eq!(expected_str, serialized);
    }
}
