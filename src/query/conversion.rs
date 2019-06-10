//! Conversion functions
use crate::{expr::Expr, query::Query};

query![ToDate, ToNumber, ToString, ToTime];

/// The `ToDate` function converts a value to a date type, if possible.
///
/// Attempting to convert a value to a date which has no date representation
/// results in an "invalid argument" error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/conversion/todate).
#[derive(Serialize, Debug, Clone)]
pub struct ToDate<'a> {
    to_date: Expr<'a>,
}

impl<'a> ToDate<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self {
            to_date: expr.into(),
        }
    }
}

/// The `ToNumber` function converts a value to a numeric literal, if possible.
///
/// Attempting to convert a value to a number which has no numeric
/// representation results in an "invalid argument" error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/conversion/tonumber).
#[derive(Serialize, Debug, Clone)]
pub struct ToNumber<'a> {
    to_number: Expr<'a>,
}

impl<'a> ToNumber<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self {
            to_number: expr.into(),
        }
    }
}

/// The `ToString` function converts a value to a string type, if possible.
///
/// Attempting to convert a value to a string which has no string representation
/// results in an "invalid argument" error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/conversion/tostring).
#[derive(Serialize, Debug, Clone)]
pub struct ToString<'a> {
    to_string: Expr<'a>,
}

impl<'a> ToString<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self {
            to_string: expr.into(),
        }
    }
}

/// The `ToTime` function converts a value to a timestamp type, if possible.
///
/// Attempting to convert a value to a timestamp which has no timestamp
/// representation results in an "invalid argument" error.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/conversion/totime).
#[derive(Serialize, Debug, Clone)]
pub struct ToTime<'a> {
    to_time: Expr<'a>,
}

impl<'a> ToTime<'a> {
    pub fn new(expr: impl Into<Expr<'a>>) -> Self {
        Self {
            to_time: expr.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_to_date() {
        let fun = ToDate::new("2019-06-06");
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "to_date": "2019-06-06",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_to_number() {
        let fun = ToNumber::new("2");
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "to_number": "2",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_to_string() {
        let fun = ToString::new(false);
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "to_string": false,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_to_time() {
        let fun = ToString::new("2015-02-20T06:30:00Z");
        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "to_string": "2015-02-20T06:30:00Z",
        });

        assert_eq!(expected, serialized);
    }
}
