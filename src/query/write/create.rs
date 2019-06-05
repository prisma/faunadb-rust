use crate::{
    expr::{Expr, Ref},
    query::Query,
};

query!(Create);

#[derive(Debug, Serialize, Clone)]
struct CreateInfo<'a>(Expr<'a>);

#[derive(Debug, Serialize, Clone)]
pub struct Create<'a> {
    create: CreateInfo<'a>,
    params: InstanceParams<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InstanceData<'a> {
    data: Expr<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: Ref<'a>, params: InstanceParams<'a>) -> Self {
        Self {
            create: CreateInfo(Expr::from(class_ref)),
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
