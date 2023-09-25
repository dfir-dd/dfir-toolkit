use std::fmt::Display;

use chrono::{NaiveDateTime, LocalResult};
use chrono_tz::Tz;
use chrono::offset::TimeZone;

pub struct ForensicsTimestamp {
    unix_ts: i64,
    src_zone: Tz,
    dst_zone: Tz,
}

impl ForensicsTimestamp {

    pub fn new(unix_ts: i64, src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            unix_ts, src_zone, dst_zone
        }
    }
}

impl Display for ForensicsTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unix_ts >= 0 {
            let src_timestamp = match self.src_zone
                .from_local_datetime(&NaiveDateTime::from_timestamp_opt(self.unix_ts, 0).unwrap())
            {
                LocalResult::None => {
                    panic!("INVALID DATETIME");
                }
                LocalResult::Single(t) => t,
                LocalResult::Ambiguous(t1, _t2) => t1,
            };
            let dst_timestamp = src_timestamp.with_timezone(&self.dst_zone);
            write!(f, "{}", dst_timestamp.to_rfc3339())
        } else {
            write!(f, "0000-00-00T00:00:00+00:00")
        }
    }
}