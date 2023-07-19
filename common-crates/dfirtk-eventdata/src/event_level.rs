use std::fmt::{Display, Write};

use anyhow::anyhow;
use evtx::SerializedEvtxRecord;
use serde::Serialize;
use serde_json::Value;

pub enum EventLevel {
    LogAlways,
    Critical,
    Error,
    Warning,
    Information,
    AuditSuccess,
    AuditFailure,
}

impl TryFrom<&SerializedEvtxRecord<Value>> for EventLevel {
    type Error = anyhow::Error;

    fn try_from(
        value: &SerializedEvtxRecord<Value>,
    ) -> Result<Self, <EventLevel as TryFrom<&SerializedEvtxRecord<Value>>>::Error> {
        match value.data["Event"]["System"]["Level"].as_u64() {
            Some(level_id) => Self::try_from(level_id),
            None => Err(anyhow!(
                "missing event level in '{data}'",
                data = value.data
            )),
        }
    }
}

impl TryFrom<u64> for EventLevel {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, <EventLevel as TryFrom<u64>>::Error> {
        Ok(match value {
            0 => EventLevel::LogAlways,
            1 => EventLevel::Critical,
            2 => EventLevel::Error,
            3 => EventLevel::Warning,
            4 => EventLevel::Information,
            5 => EventLevel::LogAlways,
            _ => return Err(anyhow!("unknown log level identifier: {value}")),
        })
    }
}

impl Display for EventLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventLevel::LogAlways => f.write_str("✍️"),
            EventLevel::Critical => f.write_char('💢'),
            EventLevel::Error => f.write_char('🔥'),
            EventLevel::Warning => f.write_str("⚠️"),
            EventLevel::Information => f.write_str("ℹ️"),
            EventLevel::AuditSuccess => f.write_char('🙂'),
            EventLevel::AuditFailure => f.write_char('😡'),
        }
    }
}

impl Serialize for EventLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EventLevel::LogAlways => serializer.serialize_str("✍️"),
            EventLevel::Critical => serializer.serialize_char('💢'),
            EventLevel::Error => serializer.serialize_char('🔥'),
            EventLevel::Warning => serializer.serialize_str("⚠️"),
            EventLevel::Information => serializer.serialize_str("ℹ️"),
            EventLevel::AuditSuccess => serializer.serialize_char('🙂'),
            EventLevel::AuditFailure => serializer.serialize_char('😡'),
        }
    }
}
