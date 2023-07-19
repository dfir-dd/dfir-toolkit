use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::{SystemField, CsvRecord};


pub struct CsvRecordBuilder<'a> {
    system_fields: &'a Vec<SystemField>,
    delimiter: char,
}

impl<'a> From<&'a Vec<SystemField>> for CsvRecordBuilder<'a> {
    fn from(system_fields: &'a Vec<SystemField>) -> Self {
        Self {
            system_fields,
            delimiter: ';'
        }
    }
}

impl<'a> CsvRecordBuilder<'a> {
    pub fn build_from_record(&self, record: SerializedEvtxRecord<Value>) -> CsvRecord<'a> {
        CsvRecord {
            record,
            delimiter: self.delimiter,
            system_fields: self.system_fields
        }
    }

    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }
}