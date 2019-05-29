use serde::{de, ser};
use std::{borrow::Cow, fmt};

pub fn serialize<'a, S>(data: &Cow<'a, [u8]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_str(&base64::encode(data))
}

pub fn deserialize<'a, 'de, D>(d: D) -> Result<Cow<'a, [u8]>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(d.deserialize_str(Base64BytesVisitor)?)
}

struct Base64BytesVisitor;

impl<'de> de::Visitor<'de> for Base64BytesVisitor {
    type Value = Cow<'static, [u8]>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("base64-encoded string of bytes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        base64::decode(value)
            .map_err(|err| de::Error::custom(err.to_string()))
            .map(|bytes| Cow::from(bytes.to_vec()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        base64::decode(value.as_str())
            .map_err(|err| de::Error::custom(err.to_string()))
            .map(|bytes| Cow::from(bytes.to_vec()))
    }
}
