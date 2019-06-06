use crate::{
    expr::{Expr, Object, Ref},
    query::Query,
};

boxed_query!(CreateKey);

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreateKey<'a> {
    create_key: KeyParams<'a>,
}

impl<'a> CreateKey<'a> {
    pub fn new(params: KeyParams<'a>) -> Self {
        Self { create_key: params }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct KeyParamsInternal<'a> {
    database: Expr<'a>,
    role: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct KeyParams<'a> {
    object: KeyParamsInternal<'a>,
}

impl<'a> KeyParams<'a> {
    pub fn new(database: Ref<'a>, role: &'a str) -> Self {
        Self {
            object: KeyParamsInternal {
                database: Expr::from(database),
                role: Expr::from(role),
                priority: None,
                data: None,
            },
        }
    }

    pub fn priority(&mut self, priority: u16) -> &mut Self {
        self.object.priority = Some(Expr::from(priority));
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_create_index() {
        let mut data = Object::default();
        data.insert("foo", "bar");

        let mut params = KeyParams::new(Ref::database("cats"), "admin");
        params.priority(420);
        params.data(data);

        let query = Query::from(CreateKey::new(params));
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "create_key": {
                "object": {
                    "database": {
                        "@ref": {
                            "class": {
                                "@ref": {
                                    "id": "databases",
                                },
                            },
                            "id": "cats",
                        },
                    },
                    "role": "admin",
                    "priority": 420,
                    "data": {
                        "object": {
                            "foo": "bar"
                        }
                    }
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
