use crate::expr::{Class, Expr, Ref};

#[derive(Debug, Serialize)]
pub struct Query<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<Create<'a>>,
    params: Expr<'a>,
}

impl<'a> Query<'a> {
    pub fn create<O>(query: Create<'a>, params: O) -> Self
    where
        O: Into<Expr<'a>>,
    {
        Self {
            create: Some(query),
            params: params.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum QueryType<'a> {
    #[serde(rename = "create")]
    Create(Create<'a>),
}

impl<'a> From<Create<'a>> for QueryType<'a> {
    fn from(create: Create<'a>) -> Self {
        QueryType::Create(create)
    }
}

#[derive(Debug, Serialize)]
pub struct Create<'a> {
    #[serde(rename = "@ref")]
    reference: Ref<'a>,
}

impl<'a> Create<'a> {
    pub fn instance(class: Class<'a>) -> Self {
        let reference = Ref::class(class.id, class);

        Self { reference }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_instance() {
        let mut params = Object::new();
        params.insert("test_field", "test_value");

        let mut data = Object::new();
        data.insert("data", params);

        let query = Query::create(Create::instance(Class::new("test")), data);
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
                    "id": "test"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
