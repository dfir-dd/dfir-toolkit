use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::es4forensics::{timestamp::Timestamp, utils::json::add_to_json};

#[derive(Serialize, Deserialize)]
pub struct NtfsFile {
    
}

impl NtfsFile {
    #[allow(dead_code)]
    pub fn documents(&self) -> impl Iterator<Item=Value> {
        let docs: HashMap<Timestamp, Value> = HashMap::new();
        docs.into_iter().map(|(ts, v)| {
            add_to_json(&v, "|@timestamp|", Value::Number(ts.timestamp_millis().into()))
        })
    }
}