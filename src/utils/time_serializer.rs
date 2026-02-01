use prost_types::Timestamp;
use serde::{Deserialize, Deserializer, Serializer};
use sqlx::types::chrono::{DateTime, TimeZone, Utc};

/// 对 Option<Timestamp> 进行序列化
pub fn serialize<S>(timestamp: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match timestamp {
        Some(ts) => {
            let dt = Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap();
            serializer.serialize_str(&dt.to_rfc3339())
        }
        None => serializer.serialize_none(),
    }
}

/// 对 Option<Timestamp> 进行反序列化
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => {
            let dt: DateTime<Utc> = str.parse().map_err(serde::de::Error::custom)?;
            Ok(Some(Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }))
        }
        None => Ok(None),
    }
}
