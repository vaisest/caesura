use chrono::{DateTime, Utc};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeStamp {
    pub datetime: DateTime<Utc>,
}

impl Serialize for TimeStamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.datetime.to_rfc3339();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for TimeStamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimeStampVisitor;

        impl<'de> Visitor<'de> for TimeStampVisitor {
            type Value = TimeStamp;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid ISO 8601 date string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let datetime = DateTime::from_str(value).map_err(de::Error::custom)?;
                Ok(TimeStamp { datetime })
            }
        }
        deserializer.deserialize_str(TimeStampVisitor)
    }
}
