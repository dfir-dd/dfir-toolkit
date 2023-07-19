use std::fmt::Display;

use clap::ValueEnum;
use eventdata::{EvtxFieldView, EventId, EventRecordId, ActivityId, RelatedActivityId, ProcessId, PROCESS_ID_MAX_LENGTH};
use evtx::SerializedEvtxRecord;
use serde_json::Value;

#[derive(ValueEnum, Clone)]
#[allow(clippy::enum_variant_names)]
pub (crate) enum SystemField {
    /// The identifier that the provider used to identify the event
    EventId,

    /// The record number assigned to the event when it was logged
    EventRecordId,

    /// A globally unique identifier that identifies the current activity. The events that are published with this identifier are part of the same activity.
    ActivityId,

    /// A globally unique identifier that identifies the activity to which control was transferred to. The related events would then have this identifier as their ActivityID identifier.
    RelatedActivityId,

    /// The ID of the process that created the event
    ProcessId
}

pub (crate) trait FilterBySystemField {
    fn filter_fields<'a>(record: &'a Self, fields: &[SystemField], ) -> anyhow::Result<Vec<Box<dyn EvtxFieldView + 'a>>>;
}

impl FilterBySystemField for SerializedEvtxRecord<Value> {
    fn filter_fields<'a>(record: &'a Self, fields: &[SystemField], ) -> anyhow::Result<Vec<Box<dyn EvtxFieldView + 'a>>> {
        let mut result: Vec<Box<dyn EvtxFieldView>> = Vec::with_capacity(fields.len());
        for field in fields {
            match field {
                SystemField::EventId => result.push(Box::new(EventId::try_from(record)?)),
                SystemField::EventRecordId => result.push(Box::new(EventRecordId::from(record))),
                SystemField::ActivityId => result.push(Box::new(ActivityId::try_from(record)?)),
                SystemField::RelatedActivityId => result.push(Box::new(RelatedActivityId::try_from(record)?)),
                SystemField::ProcessId => {
                    match ProcessId::try_from(record) {
                        Ok(f) => result.push(Box::new(f)),
                        _ => result.push(Box::new(EmptyField::with_size(PROCESS_ID_MAX_LENGTH)))    
                    }
                }
            }
        }

        Ok(result)
    }
}

struct EmptyField {
    size: usize
}

impl EmptyField {
    pub fn with_size(size: usize) -> Self {
        Self {
            size
        }
    }
}

impl Display for EmptyField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(self.size))
    }
}
impl EvtxFieldView for EmptyField {

}
