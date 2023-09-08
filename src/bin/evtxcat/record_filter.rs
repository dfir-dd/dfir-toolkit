use std::{io::{Read, Seek}, collections::HashMap, fs::File};

use evtx::{EvtxParser, SerializedEvtxRecord};

use crate::unfiltered::Unfiltered;


pub (crate) trait RecordFilter: Sized {
    type ReaderType: Read + Seek;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self>;

    fn filter_by_id(
        mut parser: EvtxParser<Self::ReaderType>,
        filter_id: u64,
    ) -> (Vec<u64>, HashMap<u64, SerializedEvtxRecord<Self>>) {
        let mut record_ids: Vec<u64> = Vec::new();
        let mut records: HashMap<u64, SerializedEvtxRecord<Self>> = HashMap::new();
        if let Some(result) = Self::unfiltered(&mut parser).find(|record| match record {
                    Ok(evt) => evt.event_record_id == filter_id,
                    _ => false,
                }) {
            let evt = result.unwrap();
            record_ids.push(evt.event_record_id);
            records.insert(evt.event_record_id, evt);
        }
        (record_ids, records)
    }

    fn filter_by_range(
        mut parser: EvtxParser<Self::ReaderType>,
        min: u64,
        max: u64,
    ) -> (Vec<u64>, HashMap<u64, SerializedEvtxRecord<Self>>) {
        let mut record_ids: Vec<u64> = Vec::new();
        let mut records: HashMap<u64, SerializedEvtxRecord<Self>> = HashMap::new();

        for record in Self::unfiltered(&mut parser) {
            match record {
                Err(_) => (),
                Ok(evt) => {
                    let id = evt.event_record_id;

                    if id >= min && id <= max {
                        record_ids.push(id);
                        records.insert(id, evt);
                    }
                }
            }
        }

        record_ids.sort_unstable();
        (record_ids, records)
    }
}

impl RecordFilter for serde_json::Value {
    type ReaderType = File;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self> {
        Unfiltered {
            inner: Box::new(parser.records_json_value()),
        }
    }
}

impl RecordFilter for String {
    type ReaderType = File;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self> {
        Unfiltered {
            inner: Box::new(parser.records()),
        }
    }
}
