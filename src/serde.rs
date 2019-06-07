//! Serde (de-)serializer functions for Fauna types.
pub mod base64_bytes;
pub mod ts_microseconds;

use chrono::offset::LocalResult;
use serde::de;
use std::fmt;

fn chrono_from<T, E, V>(me: LocalResult<T>, ts: &V) -> Result<T, E>
where
    E: de::Error,
    V: fmt::Display,
    T: fmt::Display,
{
    match me {
        LocalResult::None => Err(E::custom(format!("value is not a legal timestamp: {}", ts))),
        LocalResult::Ambiguous(min, max) => Err(E::custom(format!(
            "value is an ambiguous timestamp: {}, could be either of {}, {}",
            ts, min, max
        ))),
        LocalResult::Single(val) => Ok(val),
    }
}
