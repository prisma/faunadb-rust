use crate::expr::{Expr, Ref};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct Get<'a> {
    #[serde(flatten)]
    reference: Expr<'a>,
    #[serde(skip_serializing)]
    pub(crate) timestamp: Option<DateTime<Utc>>,
}

impl<'a> Get<'a> {
    pub fn instance(reference: Ref<'a>) -> Self {
        let reference = Expr::from(reference);
        let timestamp = None;

        Self {
            reference,
            timestamp,
        }
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp = Some(ts);
        self
    }
}
