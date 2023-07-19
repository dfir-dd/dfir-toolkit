use std::ops::Deref;

use chrono::{DateTime, NaiveDateTime, Utc, NaiveDate, NaiveTime};


/// this struct is practically a parser for
/// RFC3339-compliant strings and their abbreviated forms.
#[derive(Clone)]
pub (crate) struct Rfc3339Datetime {
    timestamp: DateTime<Utc>
}

impl From<&str> for Rfc3339Datetime {
    fn from(s: &str) -> Self {
        if let Ok(timestamp) = DateTime::parse_from_rfc3339(s) {
            return Self{timestamp: timestamp.with_timezone(&chrono::Utc)}
        }
        
        if let Ok(timestamp) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
            return Self{timestamp: DateTime::<Utc>::from_utc(timestamp, Utc)}
        }

        if let Ok(timestamp) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            return Self{timestamp: DateTime::<Utc>::from_utc(timestamp, Utc)}
        }

        if let Ok(timestamp) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let timestamp = NaiveDateTime::new(timestamp, time);
            return Self{timestamp: DateTime::<Utc>::from_utc(timestamp, Utc)}
        }

        panic!("invalid timestamp: '{s}'");
    }
}

impl Deref for Rfc3339Datetime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}