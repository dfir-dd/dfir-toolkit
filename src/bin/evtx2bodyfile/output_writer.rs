use std::io::Write;

use chrono::{DateTime, Utc};
use dfirtk_eventdata::{ActivityId, EventId, ProcessId, RelatedActivityId};
use evtx::SerializedEvtxRecord;
use flow_record::prelude::{FlowRecord, Serializer};
use flow_record::{
    prelude::{rmpv, FieldType},
    ToMsgPackValue,
};
use flow_record::derive::FlowRecord;

use crate::bf_data::BfData;

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
struct WindowsEvent {
    record_timestamp: DateTime<Utc>,
    event_id: u16,
    event_record_id: u64,
    activity_id: String,
    related_activity_id: String,
    process_id: u64,
}

impl TryFrom<&SerializedEvtxRecord<serde_json::Value>> for WindowsEvent {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<serde_json::Value>) -> Result<Self, Self::Error> {
        let record_timestamp = record.timestamp;
        let event_id = EventId::try_from(record)?.0;
        let event_record_id = record.event_record_id;
        let activity_id = ActivityId::try_from(record)?.value().to_string();
        let related_activity_id = RelatedActivityId::try_from(record)?.to_string();
        let process_id = ProcessId::try_from(record)?.0;
        Ok(Self {
            record_timestamp,
            event_id,
            event_record_id,
            activity_id,
            related_activity_id,
            process_id,
        })
    }
}
struct ValueWrapper<'v>(&'v serde_json::Value);

impl<'v> From<&'v serde_json::Value> for ValueWrapper<'v> {
    fn from(value: &'v serde_json::Value) -> Self {
        Self(value)
    }
}

impl<'v> ToMsgPackValue for ValueWrapper<'v> {
    fn to_msgpack_value(self) -> rmpv::Value {
        match self.0 {
            serde_json::Value::Null => rmpv::Value::Nil,
            serde_json::Value::Bool(_) => todo!(),
            serde_json::Value::Number(_number) => todo!(),
            serde_json::Value::String(_) => todo!(),
            serde_json::Value::Array(_vec) => todo!(),
            serde_json::Value::Object(_map) => todo!(),
        }
    }

    fn field_type() -> FieldType {
        FieldType::String
    }
}
