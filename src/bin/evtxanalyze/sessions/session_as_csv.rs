use chrono::{DateTime, Duration, Utc};
use eventdata::SessionId;
use serde::{Serialize, Serializer};

use super::ActiveDirectoryDomainName;

#[derive(Serialize, Debug)]
pub struct SessionAsCsv {
    #[serde(serialize_with = "serialize_timestamp")]
    pub begin: DateTime<Utc>,

    #[serde(serialize_with = "serialize_timestamp")]
    pub end: DateTime<Utc>,

    #[serde(serialize_with = "serialize_duration")]
    pub duration: Duration,

    pub domain: Option<ActiveDirectoryDomainName>,

    pub usernames: String,

    pub clients: String,

    pub server: Option<String>,

    pub computer: String,

    pub session_id: SessionId,

    pub events: usize,
}

fn serialize_duration<S>(duration: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{duration}"))
}


fn serialize_timestamp<S>(ts: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&ts.format("%Y-%m-%dT%H:%M:%S").to_string())
}
