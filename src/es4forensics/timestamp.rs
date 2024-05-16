use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use serde_json::{json, Value};
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Timestamp {
    ts: i64,
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.ts)
    }
}

impl<Tz> From<DateTime<Tz>> for Timestamp
where
    Tz: TimeZone,
{
    fn from(d: DateTime<Tz>) -> Self {
        let ts = d.with_timezone(&Utc);
        log::trace!("converting {:?} to {}", d, ts.timestamp_millis());
        Self {
            ts: ts.timestamp_millis(),
        }
    }
}

impl TryFrom<(i64, &Tz)> for Timestamp {
    type Error = anyhow::Error;

    fn try_from((unix_ts, src_tz): (i64, &Tz)) -> Result<Self, Self::Error> {
        let ts = match DateTime::from_timestamp(unix_ts, 0) {
            Some(ts) => ts.with_timezone(src_tz),
            None => {
                return Err(anyhow!("INVALID DATETIME"));
            }
        };
        Ok(Self {
            ts: ts.timestamp_millis(),
        })
    }
}

impl From<&Timestamp> for Value {
    fn from(ts: &Timestamp) -> Self {
        json!(ts.ts)
    }
}

impl Timestamp {
    pub fn timestamp_millis(&self) -> i64 {
        self.ts
    }
}
