use std::fmt::Display;

use chrono::format::StrftimeItems;
use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset, LocalResult, NaiveDateTime};
use chrono_tz::Tz;
use lazy_static::lazy_static;

lazy_static! {
    static ref TIMESTAMP_FORMAT: Option<String> = {
        if let Ok(format) = std::env::var("DFIR_DATE") {
            if StrftimeItems::new(&format).any(|i| i == chrono::format::Item::Error) {
                eprintln!();
                eprintln!("ERROR: invalid date format: '{format}' stored in environment variable $DFIR_DATE!");
                eprintln!();
                eprintln!("Please take a look at");
                eprintln!();
                eprintln!("        <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>");
                eprintln!();
                eprintln!("to see which format strings are accepted.");
                eprintln!();
                std::process::exit(-1);
            } else {
                Some(format)
            }
        } else {
            None
        }
    };
    static ref ZERO: DateTime<FixedOffset> =
        DateTime::<FixedOffset>::parse_from_rfc3339("0000-00-00T00:00:00+00:00").unwrap();
}

pub struct ForensicsTimestamp {
    unix_ts: i64,
    src_zone: Tz,
    dst_zone: Tz,
}

impl ForensicsTimestamp {
    pub fn new(unix_ts: i64, src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            unix_ts,
            src_zone,
            dst_zone,
        }
    }

    fn display_datetime<TZ: TimeZone>(
        dt: &DateTime<TZ>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        <TZ as TimeZone>::Offset: std::fmt::Display,
    {
        match &*TIMESTAMP_FORMAT {
            Some(format) => dt.format(format).fmt(f),
            None => dt.to_rfc3339().fmt(f),
        }
    }
}

impl Display for ForensicsTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unix_ts >= 0 {
            let src_timestamp = match self
                .src_zone
                .from_local_datetime(&NaiveDateTime::from_timestamp_opt(self.unix_ts, 0).unwrap())
            {
                LocalResult::None => {
                    panic!("INVALID DATETIME");
                }
                LocalResult::Single(t) => t,
                LocalResult::Ambiguous(t1, _t2) => t1,
            };

            Self::display_datetime(&src_timestamp.with_timezone(&self.dst_zone), f)
        } else {
            Self::display_datetime(&*ZERO, f)
        }
    }
}
