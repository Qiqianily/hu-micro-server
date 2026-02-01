use prost_types::Timestamp;
use sqlx::types::chrono::{DateTime, TimeZone, Utc};

// DateTime<Utc> -> Timestamp
pub fn datetime_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

// Option<DateTime<Utc>> -> Option<Timestamp>
pub fn option_datetime_to_timestamp(dt: Option<DateTime<Utc>>) -> Option<Timestamp> {
    dt.map(datetime_to_timestamp)
}

// Timestamp -> DateTime<Utc>
pub fn timestamp_to_datetime(ts: &Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap()
}
