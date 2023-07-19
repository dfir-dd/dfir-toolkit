use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::SystemField;

pub struct CsvRecord<'a> {
    pub (crate) record: SerializedEvtxRecord<Value>,
    pub (crate) delimiter: char,
    pub (crate) system_fields: &'a Vec<SystemField>,
}
/*
impl<'a> Serialize for CsvRecord<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.record.timestamp.to_rfc3339())?;
        let fields = <SerializedEvtxRecord<Value> as FilterBySystemField>::filter_fields(
            &self.record,
            &self.system_fields[..],
        ).unwrap();
        for field in fields.into_iter() {
            serializer.serialize_str(&field.to_string());
        }
    }
}
 */