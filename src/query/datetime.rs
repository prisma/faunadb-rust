//! Time and date functions
use crate::{expr::Expr, query::Query};

query![Date, Epoch, Time];

/// The `Date` function constructs a Date from an ISO 8601 formatted string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/time_date/date)
#[derive(Serialize, Clone, Debug)]
pub struct Date<'a> {
    date: Expr<'a>,
}

impl<'a> Date<'a> {
    pub fn new(dateish: impl Into<Expr<'a>>) -> Self {
        Self {
            date: dateish.into(),
        }
    }
}

#[derive(Serialize, Clone, Debug, Copy)]
pub enum EpochUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "millisecond")]
    Millisecond,
    #[serde(rename = "microsecond")]
    Microsecond,
    #[serde(rename = "nanosecond")]
    Nanosecond,
}

/// The `Epoch` function constructs a Timestamp relative to the epoch
/// (1970-01-01T00:00:00Z).
///
/// The num argument must be an integer value. Epoch adds num to offset defined
/// in units and returns a timestamp.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/time_date/epoch)
#[derive(Serialize, Clone, Debug)]
pub struct Epoch<'a> {
    epoch: Expr<'a>,
    unit: EpochUnit,
}

impl<'a> Epoch<'a> {
    pub fn new(num: impl Into<Expr<'a>>, unit: EpochUnit) -> Self {
        Self {
            epoch: num.into(),
            unit,
        }
    }
}

/// The `Time` function constructs a Timestamp from an ISO 8601 string.
///
/// The special string now may be used to construct a time from the current
/// request’s transaction time. Multiple references to now within the same
/// transaction produce the same timestamp. The current transaction time is the
/// same on all nodes that participate in the transaction. When doing a temporal
/// query, now means the current time of the query, not the current time.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/time_date/time)
#[derive(Serialize, Clone, Debug)]
pub struct Time<'a> {
    time: Expr<'a>,
}

impl<'a> Time<'a> {
    pub fn new(timeish: impl Into<Expr<'a>>) -> Self {
        Self {
            time: timeish.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_date() {
        let fun = Date::new("1970-01-01");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "date": "1970-01-01",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_epoch() {
        let fun = Epoch::new(5, EpochUnit::Second);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "epoch": 5,
            "unit": "second"
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_time() {
        let fun = Time::new("1970-01-01T00:00:00+00:00");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "time": "1970-01-01T00:00:00+00:00",
        });

        assert_eq!(expected, serialized);
    }
}
