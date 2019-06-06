use crate::{
    expr::{Expr, Object, Ref},
    query::Query,
};

boxed_query!(CreateKey);

#[derive(Debug, Serialize, Clone, Deserialize, Copy)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "server-readonly")]
    ServerReadOnly,
    #[serde(rename = "client")]
    Client,
}

/// `CreateKey` creates a new key to access a database with the specified
/// `param_object`. It requires an admin key for authentication.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/createkey)
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
#[doc(hidden)]
pub struct KeyParamsInternal<'a> {
    database: Expr<'a>,
    role: Role,
    priority: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct KeyParams<'a> {
    object: KeyParamsInternal<'a>,
}

impl<'a> KeyParams<'a> {
    /// A new `param_object` with the required fields:
    ///
    /// * A reference to the database for which a key should be created.
    /// * The access role
    pub fn new(database: Ref<'a>, role: Role) -> Self {
        Self {
            object: KeyParamsInternal {
                database: Expr::from(database),
                role,
                priority: Expr::from(1),
                data: None,
            },
        }
    }

    /// A relative weight between 1 and 500, inclusive, indicating how many
    /// resources this key will be allowed to utilize. Defaults to 1. A higher
    /// number means more resources.
    pub fn priority(&mut self, priority: u16) -> &mut Self {
        self.object.priority = Expr::from(priority);
        self
    }

    /// This is user-defined metadata for the key. It is provided for the
    /// developer to store information at the key level.
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

        let mut params = KeyParams::new(Ref::database("cats"), Role::Admin);
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
