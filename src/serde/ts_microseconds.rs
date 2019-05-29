use super::chrono_from;
use chrono::{offset::TimeZone, DateTime, Utc};
use serde::de;
use std::fmt;

pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(d.deserialize_i64(MicroSeondsTimestampVisitor)
        .map(|dt| dt.with_timezone(&Utc))?)
}

struct MicroSeondsTimestampVisitor;

impl<'de> de::Visitor<'de> for MicroSeondsTimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a unix timestamp in microseconds")
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_i64<E>(self, value: i64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        chrono_from(
            Utc.timestamp_opt(value / 1_000_000, ((value % 1_000_000) * 1_000) as u32),
            &value,
        )
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        chrono_from(
            Utc.timestamp_opt(
                (value / 1_000_000) as i64,
                ((value % 1_000_000) * 1_000) as u32,
            ),
            &value,
        )
    }
}
