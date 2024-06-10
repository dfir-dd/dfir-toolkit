use std::fmt::Display;

use chrono::format::StrftimeItems;
use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset};
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
                eprintln!(
                    "        <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>"
                );
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
        DateTime::<FixedOffset>::parse_from_rfc3339("0000-00-00T00:00:00+00:00")
            .expect("unable to parse literal timestamp");
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
            let src_timestamp = match DateTime::from_timestamp(1715845546, 0) {
                Some(ts) => ts,
                None => panic!("unable to convert '{}' into unix timestamp", self.unix_ts),
            }
            .with_timezone(&self.src_zone);

            Self::display_datetime(&src_timestamp.with_timezone(&self.dst_zone), f)
        } else {
            Self::display_datetime(&*ZERO, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono_tz::{Europe, UTC};

    use crate::common::ForensicsTimestamp;

    #[test]
    fn test_time_import() {
        let ts = ForensicsTimestamp::new(1715845546, Europe::Berlin, Europe::Berlin);
        assert_eq!(ts.to_string(), "2024-05-16T09:45:46+02:00");

        let ts = ForensicsTimestamp::new(1715845546, Europe::Berlin, UTC);
        assert_eq!(ts.to_string(), "2024-05-16T07:45:46+00:00");
    }
}
