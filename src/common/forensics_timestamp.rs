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

    pub fn format_date(&self) -> String {
        if self.unix_ts >= 0 {
            let src_timestamp = match self.src_zone
                .from_local_datetime(&NaiveDateTime::from_timestamp_opt(self.unix_ts, 0).unwrap())
            {
                LocalResult::None => {
                    return "INVALID DATETIME".to_owned();
                }
                LocalResult::Single(t) => t,
                LocalResult::Ambiguous(t1, _t2) => t1,
            };
            let dst_timestamp = src_timestamp.with_timezone(&self.dst_zone);
            dst_timestamp.to_rfc3339()
        } else {
            "0000-00-00T00:00:00+00:00".to_owned()
        }
    }
}