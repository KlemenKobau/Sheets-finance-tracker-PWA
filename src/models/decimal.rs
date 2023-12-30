use std::str::FromStr;

use fraction::Decimal;
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};
use tracing::warn;

pub struct SerializableDecimal(pub Decimal);

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = SerializableDecimal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A string representing a decimal number")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Decimal::from_str(value)
            .map(SerializableDecimal)
            .map_err(|x| {
                warn!("Error: {x}, value: {value}");
                E::custom(x)
            })
    }
}

impl Serialize for SerializableDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let decimal_string = self.0.to_string();
        serializer.serialize_str(&decimal_string)
    }
}

impl<'de> Deserialize<'de> for SerializableDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}
