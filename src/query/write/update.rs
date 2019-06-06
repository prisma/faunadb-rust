use crate::{
    expr::{Expr, Ref},
    query::Query,
};

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
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Update<'a> {
    update: Expr<'a>,
    params: UpdateParams<'a>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct UpdateParams<'a> {
    object: UpdateObject<'a>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
#[doc(hidden)]
pub struct UpdateObject<'a> {
    data: Expr<'a>,
    credentials: Expr<'a>,
    delegates: Expr<'a>,
}

impl<'a> Update<'a> {
    pub fn new(reference: Ref<'a>, params: UpdateParams<'a>) -> Self {
        Update {
            update: Expr::from(reference),
            params,
        }
    }
}

impl<'a> UpdateParams<'a> {
    pub fn new(
        data: impl Into<Expr<'a>>,
        credentials: impl Into<Expr<'a>>,
        delegates: impl Into<Expr<'a>>,
    ) -> Self {
        Self {
            object: UpdateObject {
                data: data.into(),
                credentials: credentials.into(),
                delegates: delegates.into(),
            },
        }
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

        let params = UpdateParams::new(data, credentials, delegates);

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
