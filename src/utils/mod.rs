use chrono::{DateTime, Utc};
use diesel::data_types::PgTimestamp;
use uuid::Uuid;

pub fn new_uuid() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

pub fn time_now() -> DateTime<Utc> {
    Utc::now()
}

pub fn pgtimestamp_to_datetime(ts: PgTimestamp) -> DateTime<Utc> {
    match DateTime::from_timestamp_micros(ts.0) {
        Some(t) => t,
        None => time_now(),
    }
}
