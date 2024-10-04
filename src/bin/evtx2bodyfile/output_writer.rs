use std::io::Write;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use dfirtk_eventdata::{ActivityId, EventId, ProcessId, RelatedActivityId};
use evtx::SerializedEvtxRecord;
use flow_record::prelude::{FlowRecord, Serializer};

use flow_record::derive::FlowRecord;

use crate::bf_data::BfData;
use crate::value_map::ValueMap;

#[derive(Default)]
pub(crate) struct BodyfileOutputWriter<W: Write>(W);
pub(crate) struct RecordOutputWriter<W: Write>(Serializer<W>);

pub(crate) trait OutputWriter<W>: From<W>
where
    W: Write,
{
    fn output(&mut self, record: &SerializedEvtxRecord<serde_json::Value>) -> anyhow::Result<()>;
}

impl<W> From<W> for BodyfileOutputWriter<W>
where
    W: Write,
{
    fn from(writer: W) -> Self {
        Self(writer)
    }
}

impl<W> From<W> for RecordOutputWriter<W>
where
    W: Write,
{
    fn from(writer: W) -> Self {
        Self(Serializer::new(writer))
    }
}

impl<W> OutputWriter<W> for BodyfileOutputWriter<W>
where
    W: Write,
{
    fn output(&mut self, record: &SerializedEvtxRecord<serde_json::Value>) -> anyhow::Result<()> {
        let bf_data = BfData::try_from(record)?;
        let s = bf_data.try_into_mactime()?;
        writeln!(self.0, "{s}")?;
        Ok(())
    }
}

impl<W> OutputWriter<W> for RecordOutputWriter<W>
where
    W: Write,
{
    fn output(&mut self, record: &SerializedEvtxRecord<serde_json::Value>) -> anyhow::Result<()> {
        let event = WindowsEvent::try_from(record)?;
        self.0.serialize(event)?;
        Ok(())
    }
}

#[derive(FlowRecord)]
#[flow_record(version = 1, source = "evtx2bodyfile", classification = "evtx")]
struct WindowsEvent<'v> {
    record_timestamp: DateTime<Utc>,
    event_id: u16,
    event_record_id: u64,
    activity_id: String,
    related_activity_id: String,
    process_id: u64,
    channel: Option<&'v str>,
    provider_name: Option<&'v str>,
    provider_guid: Option<&'v str>,
    level: u64,
    computer: Option<&'v str>,
    event_source_name: Option<&'v str>,

    event_data: Option<ValueMap<'v>>,
    user_data: Option<ValueMap<'v>>,
}

impl<'v> TryFrom<&'v SerializedEvtxRecord<serde_json::Value>> for WindowsEvent<'v> {
    type Error = anyhow::Error;

    fn try_from(record: &'v SerializedEvtxRecord<serde_json::Value>) -> Result<Self, Self::Error> {
        let record_timestamp = record.timestamp;
        let event_id = EventId::try_from(record)?.0;
        let event_record_id = record.event_record_id;
        let activity_id = ActivityId::try_from(record)?.value().to_string();
        let related_activity_id = RelatedActivityId::try_from(record)?.to_string();
        let process_id = ProcessId::try_from(record)?.0;

        let event = record
            .data
            .get("Event")
            .ok_or_else(|| anyhow!("missing 'Event' entry"))?;
        let event_data = event.opt_value("/EventData");
        let user_data = event.opt_value("/UserData");

        let channel = event.opt_string("/System/Channel");
        let computer = event.opt_string("/System/Computer");
        let level = event.number("/System/Level")?;
        let provider_name = event.opt_string("/System/Provider/#attributes/Name");
        let provider_guid = event.opt_string("/System/Provider/#attributes/Guid");
        let event_source_name = event.opt_string("/System/Provider/#attributes/EventSourceName");

        Ok(Self {
            record_timestamp,
            event_id,
            event_record_id,
            activity_id,
            related_activity_id,
            process_id,
            event_data,
            user_data,
            channel,
            provider_name,
            provider_guid,
            level,
            computer,
            event_source_name,
        })
    }
}

trait EvtxValues {
    fn opt_value(&self, id: &str) -> Option<ValueMap<'_>>;
    fn opt_string(&self, id: &str) -> Option<&str>;
    fn number(&self, id: &str) -> anyhow::Result<u64>;
}

impl EvtxValues for serde_json::Value {
    fn opt_value(&self, id: &str) -> Option<ValueMap<'_>> {
        self.pointer(id).map(ValueMap::from)
    }

    fn opt_string(&self, id: &str) -> Option<&str> {
        self.pointer(id)
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(&s[..]),
                _ => None,
            })
    }

    fn number(&self, id: &str) -> anyhow::Result<u64> {
        self.pointer(id)
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("missing 'Level' entry"))
    }
}
