use crate::{
    expr::{Expr, Ref},
    query::{write::Action, Query},
};
use chrono::{DateTime, Utc};

query!(Insert);

/// The Insert function adds an event to an instance’s history at a specified
/// timestamp.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/write/insert)
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Insert<'a> {
    insert: Expr<'a>,
    #[serde(rename = "ts")]
    timestamp: Expr<'a>,
    action: Action,
    params: InsertParams<'a>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct InsertParams<'a> {
    object: InsertObject<'a>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct InsertObject<'a> {
    data: Expr<'a>,
    credentials: Expr<'a>,
    delegates: Expr<'a>,
}

impl<'a> Insert<'a> {
    pub fn new(
        reference: Ref<'a>,
        timestamp: DateTime<Utc>,
        action: Action,
        params: InsertParams<'a>,
    ) -> Self {
        Insert {
            insert: Expr::from(reference),
            timestamp: Expr::from(timestamp),
            action,
            params,
        }
    }
}

impl<'a> InsertParams<'a> {
    pub fn new(
        data: impl Into<Expr<'a>>,
        credentials: impl Into<Expr<'a>>,
        delegates: impl Into<Expr<'a>>,
    ) -> Self {
        Self {
            object: InsertObject {
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
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_insert() {
        let mut data = Object::default();
        data.insert("scratch", "moar");

        let mut credentials = Object::default();
        credentials.insert("push", "meowmeow");

        let mut delegates = Object::default();
        delegates.insert("pawpaw", "meow");

        let params = InsertParams::new(data, credentials, delegates);

        let fun = Insert::new(
            Ref::instance("musti"),
            Utc.timestamp(60, 0),
            Action::Update,
            params,
        );

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
            "ts": {"@ts": "1970-01-01T00:01:00Z"},
            "action": "update",
            "insert": {
                "@ref": {
                    "id": "musti"
                }
            }
        });

        assert_eq!(expected, serialized);
    }
}
