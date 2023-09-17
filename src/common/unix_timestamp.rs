use chrono::{DateTime, Utc};


pub trait UnixTimestamp {
    fn ts2date(ts: i64) -> String;
}

impl UnixTimestamp for DateTime<Utc> {
    fn ts2date(ts: i64) -> String {
        let dt = DateTime::<Utc>::from_timestamp(ts,0).expect("invalid timestamp");
        dt.to_string()
    }
}