use evtx::SerializedEvtxRecord;
use serde_json::Value;

use crate::bf_data::BfData;

#[derive(Default)]
pub(crate) struct JsonOutputFormatter;
#[derive(Default)]
pub(crate) struct BodyfileOutputFormatter;

pub(crate) trait OutputFormatter {
    fn record_to_string(&self, record: &SerializedEvtxRecord<Value>) -> anyhow::Result<String>;
}

impl OutputFormatter for JsonOutputFormatter {
    fn record_to_string(&self, record: &SerializedEvtxRecord<Value>) -> anyhow::Result<String> {
        let bf_data = BfData::try_from(record)?;
        bf_data.try_into_json()
    }
}

impl OutputFormatter for BodyfileOutputFormatter {
    fn record_to_string(&self, record: &SerializedEvtxRecord<Value>) -> anyhow::Result<String> {
        let bf_data = BfData::try_from(record)?;
        bf_data.try_into_mactime()
    }
}
