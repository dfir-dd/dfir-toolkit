use std::fmt::Display;

use chrono::format::StrftimeItems;
use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use serde::Serialize;

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
    timestamp: DateTime<Utc>,
    dst_zone: Tz,
}

impl From<i64> for ForensicsTimestamp {
    fn from(value: i64) -> Self {
        let timestamp = match DateTime::from_timestamp(value, 0) {
            Some(ts) => ts,
            None => panic!("unable to convert '{value}' into unix timestamp"),
        };
        Self {
            timestamp,
            dst_zone: Tz::UTC,
        }
    }
}

impl ForensicsTimestamp {
    pub fn new(unix_ts: i64, dst_zone: Tz) -> Self {
        let timestamp = match DateTime::from_timestamp(unix_ts, 0) {
            Some(ts) => ts,
            None => panic!("unable to convert '{unix_ts}' into unix timestamp"),
        };
        Self {
            timestamp,
            dst_zone,
        }
    }

    pub fn with_timezone(mut self, dst_zone: Tz) -> Self {
        self.dst_zone = dst_zone;
        self
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
        Self::display_datetime(&self.timestamp.with_timezone(&self.dst_zone), f)
    }
}

impl Serialize for ForensicsTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&format!("{self}"))
    }
}

#[cfg(test)]
mod tests {
    use chrono_tz::{Europe, UTC};

    use crate::common::ForensicsTimestamp;

    #[test]
    fn test_time_import() {
        let ts = ForensicsTimestamp::from(1715845546).with_timezone(Europe::Berlin);
        assert_eq!(ts.to_string(), "2024-05-16T09:45:46+02:00");

        let ts = ForensicsTimestamp::from(1715845546).with_timezone(UTC);
        assert_eq!(ts.to_string(), "2024-05-16T07:45:46+00:00");
    }
}
