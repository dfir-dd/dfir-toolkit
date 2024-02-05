use std::fmt::Display;

use chrono::{DateTime, FixedOffset, TimeZone};
use lazy_static::lazy_static;

lazy_static! {
    static ref TIMESTAMP_FORMAT: Option<String> = std::env::var("DFIR_DATE").ok();
    static ref ZERO: DateTime<FixedOffset> =
        DateTime::<FixedOffset>::parse_from_rfc3339("0000-00-00T00:00:00+00:00").unwrap();
}

/// Wrapper around [`DateTime`] to allow customization of the timestamp output
/// using the `DFIR_DATE` environment variable
/// 
pub struct FormattableDatetime<TZ: TimeZone>(DateTime<TZ>)
where
    <TZ as TimeZone>::Offset: std::fmt::Display;

impl<TZ> From<DateTime<TZ>> for FormattableDatetime<TZ>
where
    TZ: TimeZone,
    <TZ as TimeZone>::Offset: std::fmt::Display,
{
    fn from(value: DateTime<TZ>) -> Self {
        Self(value)
    }
}

impl<TZ> From<&DateTime<TZ>> for FormattableDatetime<TZ>
where
    TZ: TimeZone,
    <TZ as TimeZone>::Offset: std::fmt::Display,
{
    fn from(value: &DateTime<TZ>) -> Self {
        Self(value.clone())
    }
}

impl<TZ> Display for FormattableDatetime<TZ>
where
    TZ: TimeZone,
    <TZ as TimeZone>::Offset: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*TIMESTAMP_FORMAT {
            Some(format) => self.0.format(format).fmt(f),
            None => self.0.to_rfc3339().fmt(f),
        }
    }
}
