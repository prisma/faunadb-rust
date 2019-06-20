use crate::{expr::Expr, query::Query};

query!(Update);

/// The `Update` operation only modifies the specified fields in the instances
/// pointed to by `ref`.
///
/// Updates are partial, and only modify values that are specified in the
/// param_object. Changes to scalar values and arrays are entirely replaced by
/// the new data. Modifications to objects are merged. Setting a value to `null`
/// completely removes the value. Fields in the instance not specified in the
/// `param_object` are not modified.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/update)
#[derive(Serialize, Debug, Clone)]
pub struct Update<'a> {
    update: Expr<'a>,
    params: UpdateParams<'a>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct UpdateParams<'a> {
    object: UpdateObject<'a>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[doc(hidden)]
pub struct UpdateObject<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegates: Option<Expr<'a>>,
}

impl<'a> Update<'a> {
    pub fn new(reference: impl Into<Expr<'a>>, params: UpdateParams<'a>) -> Self {
        Update {
            update: reference.into(),
            params,
        }
    }
}

impl<'a> UpdateParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(&mut self, data: impl Into<Expr<'a>>) -> &mut Self {
        self.object.data = Some(data.into());
        self
    }

    pub fn credentials(&mut self, credentials: impl Into<Expr<'a>>) -> &mut Self {
        self.object.credentials = Some(credentials.into());
        self
    }

    pub fn delegates(&mut self, delegates: impl Into<Expr<'a>>) -> &mut Self {
        self.object.delegates = Some(delegates.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_insert() {
        let mut data = Object::default();
        data.insert("scratch", "moar");

        let mut credentials = Object::default();
        credentials.insert("push", "meowmeow");

        let mut delegates = Object::default();
        delegates.insert("pawpaw", "meow");

        let mut params = UpdateParams::new();
        params.data(data);
        params.credentials(credentials);
        params.delegates(delegates);

        let fun = Update::new(Ref::instance("musti"), params);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "params": {
                "object": {
                    "data": {
                        "object": {
                            "scratch": "moar"
                        }
                    },
                    "credentials": {
                        "object": {
                            "push": "meowmeow"
                        }
                    },
                    "delegates": {
                        "object": {
                            "pawpaw": "meow"
                        }
                    },
                }
            },
            "update": {
                "@ref": {
                    "id": "musti"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
