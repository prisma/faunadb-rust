use crate::{
    expr::{Expr, IndexPermission, Object, Ref},
    query::Query,
};
use std::borrow::Cow;

boxed_query!(CreateIndex);

#[derive(Debug, Serialize, Clone)]
pub struct CreateIndex<'a> {
    create_index: IndexParams<'a>,
}

impl<'a> CreateIndex<'a> {
    pub fn new(params: IndexParams<'a>) -> Self {
        Self {
            create_index: params,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexField<'a>(Vec<Cow<'a, str>>);

#[derive(Debug, Serialize, Clone)]
pub struct IndexBinding<'a>(Cow<'a, str>);

#[derive(Debug, Serialize, Clone)]
pub enum TermObject<'a> {
    #[serde(rename = "field")]
    Field(IndexField<'a>),
    #[serde(rename = "binding")]
    Binding(IndexBinding<'a>),
}

#[derive(Debug, Serialize, Clone)]
pub struct Term<'a> {
    object: TermObject<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ValueObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<IndexField<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binding: Option<IndexBinding<'a>>,
    reverse: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct Value<'a> {
    object: ValueObject<'a>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexParamsInternal<'a> {
    name: Cow<'a, str>,
    source: Expr<'a>,
    active: bool,
    unique: bool,
    serialized: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms: Option<Vec<Term<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<Value<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partitions: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<IndexPermission<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexParams<'a> {
    object: IndexParamsInternal<'a>,
}

impl<'a> Term<'a> {
    pub fn field<T>(path: Vec<T>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let field = IndexField(path.into_iter().map(Into::into).collect());

        Self {
            object: TermObject::Field(field),
        }
    }

    pub fn binding<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let binding = IndexBinding(name.into());

        Self {
            object: TermObject::Binding(binding),
        }
    }
}

impl<'a> Value<'a> {
    pub fn field<T>(path: Vec<T>) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let field = IndexField(path.into_iter().map(Into::into).collect());

        Self {
            object: ValueObject {
                field: Some(field),
                binding: None,
                reverse: false,
            },
        }
    }

    pub fn binding<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let binding = IndexBinding(name.into());

        Self {
            object: ValueObject {
                field: None,
                binding: Some(binding),
                reverse: false,
            },
        }
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.object.reverse = true;
        self
    }
}

impl<'a> IndexParams<'a> {
    pub fn new<S>(name: S, source: Ref<'a>) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            object: IndexParamsInternal {
                name: name.into(),
                source: Expr::from(source),
                active: false,
                unique: false,
                serialized: false,
                terms: None,
                values: None,
                partitions: None,
                permissions: None,
                data: None,
            },
        }
    }

    pub fn active(&mut self) -> &mut Self {
        self.object.active = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.object.unique = true;
        self
    }

    pub fn serialized(&mut self) -> &mut Self {
        self.object.serialized = true;
        self
    }

    pub fn terms(&mut self, terms: Vec<Term<'a>>) -> &mut Self {
        self.object.terms = Some(terms);
        self
    }

    pub fn values(&mut self, values: Vec<Value<'a>>) -> &mut Self {
        self.object.values = Some(values);
        self
    }

    pub fn partitions(&mut self, partitions: u16) -> &mut Self {
        self.object.partitions = Some(partitions);
        self
    }

    pub fn permissions(&mut self, permissions: IndexPermission<'a>) -> &mut Self {
        self.object.permissions = Some(permissions);
        self
    }

    pub fn data(&mut self, data: Object<'a>) -> &mut Self {
        self.object.data = Some(Expr::from(data));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use serde_json::{self, json};

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
}
