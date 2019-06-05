use crate::{
    expr::{Expr, Ref},
    query::Query,
};
use chrono::{DateTime, Utc};

query!(Get);

#[derive(Debug, Serialize, Clone)]
struct GetObject<'a>(Expr<'a>);

#[derive(Debug, Serialize, Clone)]
pub struct Get<'a> {
    get: GetObject<'a>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<Expr<'a>>,
}

impl<'a> Get<'a> {
    pub fn instance(reference: Ref<'a>) -> Self {
        Self {
            get: GetObject(Expr::from(reference)),
            timestamp: None,
        }
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp = Some(Expr::from(ts));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{offset::TimeZone, Utc};
    use serde_json::{self, json};

    #[test]
    fn test_get() {
        let mut get = Get::instance(Ref::instance("musti"));
        get.timestamp(Utc.timestamp(60, 0));

        let query = Query::from(get);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "get": {
                "@ref": {
                    "id": "musti"
                }
            },
            "ts": {
                "@ts": Utc.timestamp(60, 0)
            }
        });

        assert_eq!(expected, serialized);
    }
}
