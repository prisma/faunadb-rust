use crate::{
    expr::{Expr, Ref},
    query::Query,
};
use chrono::{DateTime, Utc};

query![Get, KeyFromSecret, Paginate];

/// The `Get` function retrieves a single instance identified by `ref`.
///
/// An optional `timestamp` can be provided to retrieve the instance which
/// existed at the specific date and time. If the timestamp is omitted the
/// default is the current time.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/read/get)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Get<'a> {
    get: Expr<'a>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<Expr<'a>>,
}

impl<'a> Get<'a> {
    pub fn instance(reference: Ref<'a>) -> Self {
        Self {
            get: Expr::from(reference),
            timestamp: None,
        }
    }

    pub fn timestamp(&mut self, ts: DateTime<Utc>) -> &mut Self {
        self.timestamp = Some(Expr::from(ts));
        self
    }
}

/// The `KeyFromSecret` function retrieves a key instance given a key’s secret string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/read/keyfromsecret)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct KeyFromSecret<'a> {
    key_from_secret: Expr<'a>,
}

impl<'a> KeyFromSecret<'a> {
    pub fn new(secret: &'a str) -> Self {
        Self {
            key_from_secret: Expr::from(secret),
        }
    }
}

/// The `Paginate` function simplifies the traversal of a query’s results.
///
/// It is best utilized when the result of a query returns more than one object
/// or an unknown number of objects. It provides cursor like semantics allowing
/// the caller to walk both forward and backward in configurable sized pages
/// through the results.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/read/paginate)
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Paginate<'a> {
    paginate: Expr<'a>,
    size: u32,
    events: bool,
    sources: bool,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Expr<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Expr<'a>>,
}

impl<'a> Paginate<'a> {
    /// Input `Set` or a `Ref`
    pub fn new(input: impl Into<Expr<'a>>) -> Self {
        Self {
            paginate: input.into(),
            size: 64,
            events: false,
            sources: false,
            timestamp: None,
            after: None,
            before: None,
        }
    }

    /// Maximum results to return in a single page. Default: `64`.
    pub fn size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }

    /// If `true`, return a page from the event history of the set. Default:
    /// `false`.
    pub fn events(&mut self, events: bool) -> &mut Self {
        self.events = events;
        self
    }

    /// If `true`, includes the source of truth providing why this object was
    /// included in the result set. Default: `false`.
    pub fn sources(&mut self, sources: bool) -> &mut Self {
        self.sources = sources;
        self
    }

    /// Return the result set at the specified point in time.
    pub fn timestamp(&mut self, timestamp: impl Into<Expr<'a>>) -> &mut Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    /// Return the next page of results after this cursor (inclusive).
    ///
    /// Cursor may be one of:
    ///
    /// * An `Integer` representing a timestamp.
    /// * A `@ts` value.
    /// * A `@date` value. Dates are interpreted as midnight on that date, in UTC.
    /// * An partial Event object: `ts`, `ts` and `action`, or all of `ts`,
    ///   `action`, and `resource` must be specified.
    pub fn after(&mut self, after: impl Into<Expr<'a>>) -> &mut Self {
        self.after = Some(after.into());
        self
    }

    /// Return the previous page of results before this cursor (exclusive).
    ///
    /// Cursor may be one of:
    ///
    /// * An `Integer` representing a timestamp.
    /// * A `@ts` value.
    /// * A `@date` value. Dates are interpreted as midnight on that date, in UTC.
    /// * An partial Event object: `ts`, `ts` and `action`, or all of `ts`,
    ///   `action`, and `resource` must be specified.
    pub fn before(&mut self, before: impl Into<Expr<'a>>) -> &mut Self {
        self.before = Some(before.into());
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

    #[test]
    fn test_key_from_secret() {
        let fun = KeyFromSecret::new("Hunter2");
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "key_from_secret": "Hunter2"
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_paginate() {
        let mut fun = Paginate::new(Classes::all());
        fun.before(Utc.timestamp(100, 0));
        fun.after(Utc.timestamp(60, 0));

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "paginate": { "classes": null },
            "after": { "@ts": "1970-01-01T00:01:00Z" },
            "before": { "@ts": "1970-01-01T00:01:40Z" },
            "size": 64,
            "sources": false,
            "events": false,
        });

        assert_eq!(expected, serialized);
    }
}
