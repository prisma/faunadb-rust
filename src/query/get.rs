use crate::expr::{Expr, Ref};
use chrono::{DateTime, Utc};

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
