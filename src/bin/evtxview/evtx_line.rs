use cursive_table_view::TableViewItem;
use dfirtk_eventdata::EventId;
use evtx::SerializedEvtxRecord;
use serde_json::Value;

use crate::evtx_column::EvtxColumn;

#[derive(Clone, Debug)]
pub struct EvtxLine {
    record: SerializedEvtxRecord<Value>,
}

impl From<SerializedEvtxRecord<Value>> for EvtxLine {
    fn from(record: SerializedEvtxRecord<Value>) -> Self {
        Self {
            record
        }
    }
}

impl TableViewItem<EvtxColumn> for EvtxLine {
    fn to_column(&self, column: EvtxColumn) -> String {
        match column {
            EvtxColumn::Timestamp => {
                self.record.timestamp.to_rfc3339()
            },
            EvtxColumn::EventRecordId => self.record.event_record_id.to_string(),
            EvtxColumn::EventId => EventId::try_from(&self.record).ok().map(|e| e.to_string()).unwrap_or_default(),
        }
    }

    fn cmp(&self, other: &Self, column: EvtxColumn) -> std::cmp::Ordering
    where
        Self: Sized {
            self.to_column(column).cmp(&other.to_column(column))
    }
}
