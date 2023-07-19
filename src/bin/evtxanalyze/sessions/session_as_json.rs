use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Serializer};

use eventdata::SessionId;

#[derive(Serialize, Debug)]
pub struct SessionAsJson {
    pub begin: DateTime<Utc>,
    pub end: DateTime<Utc>,

    #[serde(serialize_with = "serialize_duration")]
    pub duration: Duration,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub usernames: HashSet<String>,

    pub session_id: SessionId,

    pub events: usize,
}

fn serialize_duration<S>(duration: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{duration}"))
}
