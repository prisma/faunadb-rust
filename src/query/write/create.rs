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
pub struct InstanceParams<'a> {
    object: InstanceData<'a>,
}

impl<'a> Create<'a> {
    pub fn new(class_ref: impl Into<Expr<'a>>, params: InstanceParams<'a>) -> Self {
        Self {
            create: class_ref.into(),
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
    use crate::{prelude::*, test_utils::*};
    use chrono::{offset::TimeZone, NaiveDate, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_create_expr() {
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

        let params = InstanceParams::new(obj);

        with_class(|class_name| {
            let response = CLIENT
                .query(Create::new(Class::find(class_name), params))
                .unwrap();
            let res = response.resource.as_object().unwrap();

            let data = res.get("data").and_then(|res| res.as_object()).unwrap();
            let name = data.get("name").and_then(|res| res.as_str());
            let id = data.get("id").and_then(|res| res.as_u64());
            let age = data.get("age").and_then(|res| res.as_u64());
            let bytes = data.get("byte_data").and_then(|res| res.as_bytes());
            let nicknames = data.get("nicknames").and_then(|res| res.as_array());
            let bool_val = data.get("am_i_cute").and_then(|res| res.as_bool());
            let ts_val = data.get("created_at").and_then(|res| res.as_timestamp());
            let date_val = data.get("birthday").and_then(|res| res.as_date());

            assert_eq!(name, Some("Musti"));
            assert_eq!(id, Some(1));
            assert_eq!(age, Some(7));
            assert_eq!(bytes, Some(&Bytes::from(vec![0x1, 0x2, 0x3])));

            assert_eq!(
                nicknames,
                Some(&nickname_vals.into_iter().map(Value::from).collect())
            );

            assert_eq!(ts_val, Some(Utc.timestamp(60, 0)));
            assert_eq!(date_val, Some(NaiveDate::from_ymd(2011, 7, 7)));
            assert_eq!(bool_val, Some(true));
        });
    }
}
