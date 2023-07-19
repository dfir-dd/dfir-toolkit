use std::fmt::Display;

use evtx::SerializedEvtxRecord;

use super::EvtxFieldView;

#[derive(PartialEq, Eq, Clone)]
pub struct EventRecordId (u64);

impl<T> From<&SerializedEvtxRecord<T>> for EventRecordId {
    fn from(value: &SerializedEvtxRecord<T>) -> Self {
        Self(value.event_record_id)
    }
}

// normally, we'd expect to have the maximum length of u64, but evtx does only store 32bit values,
// so 10 characters should be fine
const EVENT_RECORD_ID_MAX_LENGTH: usize = 10;
impl EvtxFieldView for EventRecordId {
    fn maximum_display_length(&self) -> usize {
        EVENT_RECORD_ID_MAX_LENGTH
    }

    fn value_with_padding(&self) -> String {
        format!("{:10}", self.0)
    }
}

impl From<EventRecordId> for u64 {
    fn from(me: EventRecordId) -> Self {
        me.0
    }
}

impl From<u64> for EventRecordId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl Display for EventRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}