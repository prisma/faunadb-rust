use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query!(Create);

/// The `Create` function adds a new instance to a class.
///
/// The `class_ref` parameter indicates what class of instance should be
/// created, while `params` contains the instance data and optional metadata.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/create)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Create<'a> {
    create: Expr<'a>,
    params: InstanceParams<'a>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[doc(hidden)]
pub struct InstanceData<'a> {
    data: Expr<'a>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: Ref<'a>, params: InstanceParams<'a>) -> Self {
        Self {
            create: Expr::from(class_ref),
            params,
        }
    }
}

impl<'a> InstanceParams<'a> {
    pub fn new<E>(data: E) -> Self
    where
        E: Into<Expr<'a>>,
    {
        Self {
            object: InstanceData { data: data.into() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
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
}
