use std::fmt::Display;
use anyhow::Result;

use chrono::{NaiveDateTime, LocalResult, DateTime, Utc};
use chrono_tz::Tz;
use chrono::offset::TimeZone;

pub struct ForensicsTimestamp {
    unix_ts: i64,
    src_timestamp: DateTime<Utc>,
    dst_zone: Tz,
}

impl ForensicsTimestamp {

    pub fn new(unix_ts: i64, src_zone: Tz, dst_zone: Tz) -> Result<Self> {
        Ok (Self {
            unix_ts, dst_zone,
            
            src_timestamp : match src_zone.from_local_datetime(&NaiveDateTime::from_timestamp_opt(unix_ts, 0).unwrap())
            {
                LocalResult::None => {
                    panic!("INVALID DATETIME");
                }
                LocalResult::Single(t) => t.with_timezone(&Utc),
                LocalResult::Ambiguous(t1, _t2) => t1.with_timezone(&Utc),
            },
            
        })
    }
}

impl Display for ForensicsTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unix_ts >= 0 {
            let dst_timestamp = self.src_timestamp.with_timezone(&self.dst_zone);
            write!(f, "{}", dst_timestamp.to_rfc3339())
        } else {
            write!(f, "0000-00-00T00:00:00+00:00")
        }
    }
}