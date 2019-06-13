use crate::{expr::Expr, query::Query};

query!(Create);

/// The `Create` function adds a new instance to a class.
///
/// The `class_ref` parameter indicates what class of instance should be
/// created, while `params` contains the instance data and optional metadata.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/create)
#[derive(Debug, Serialize, Clone)]
pub struct Create<'a> {
    create: Expr<'a>,
    params: InstanceParams<'a>,
}

#[derive(Debug, Serialize, Clone)]
#[doc(hidden)]
pub struct InstanceData<'a> {
    data: Expr<'a>,
}

#[derive(Debug, Serialize, Clone)]
struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: impl Into<Expr<'a>>, data: impl Into<Expr<'a>>) -> Self {
        Self {
            create: class_ref.into(),
            params: InstanceParams::new(data),
        }
    }
}

impl<'a> InstanceParams<'a> {
    fn new<E>(data: E) -> Self
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
    use crate::{prelude::*, test_utils::*};
    use chrono::{offset::TimeZone, NaiveDate, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_create_expr() {
        let mut obj = Object::default();
        obj.insert("test_field", "test_value");

        let query = Query::from(Create::new(Ref::class("test"), obj));
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

    #[test]
    fn test_create_eval() {
        let mut obj = Object::default();
        let nickname_vals = vec!["mustu", "muspus", "mustikka"];

        obj.insert("name", "Musti");
        obj.insert("id", 1);
        obj.insert("age", 7);
        obj.insert("byte_data", Bytes::from(vec![0x1, 0x2, 0x3]));
        obj.insert("nicknames", Array::from(nickname_vals.clone()));
        obj.insert("am_i_cute", true);
        obj.insert("created_at", Utc.timestamp(60, 0));
        obj.insert("birthday", NaiveDate::from_ymd(2011, 7, 7));

        with_class(|class_name| {
            let response = CLIENT
                .query(Create::new(Class::find(class_name), obj))
                .unwrap();

            let res = response.resource;

            assert_eq!(res["data"]["name"].as_str(), Some("Musti"));
            assert_eq!(res["data"]["id"].as_u64(), Some(1));
            assert_eq!(res["data"]["age"].as_u64(), Some(7));
            assert_eq!(res["data"]["am_i_cute"].as_bool(), Some(true));

            assert_eq!(
                res["data"]["byte_data"].as_bytes(),
                Some(&Bytes::from(vec![0x1, 0x2, 0x3]))
            );

            assert_eq!(
                res["data"]["created_at"].as_timestamp(),
                Some(Utc.timestamp(60, 0))
            );

            assert_eq!(
                res["data"]["birthday"].as_date(),
                Some(NaiveDate::from_ymd(2011, 7, 7))
            );

            assert_eq!(res["data"]["nicknames"][0].as_str(), Some("mustu"));
            assert_eq!(res["data"]["nicknames"][1].as_str(), Some("muspus"));
            assert_eq!(res["data"]["nicknames"][2].as_str(), Some("mustikka"));
        });
    }
}
