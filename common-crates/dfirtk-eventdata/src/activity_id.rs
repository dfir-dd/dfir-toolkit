use std::fmt::Display;

use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::EvtxFieldView;

#[derive(PartialEq, Eq, Clone)]
pub struct ActivityId<'a>(&'a Value);

impl<'a> TryFrom<&'a SerializedEvtxRecord<Value>> for ActivityId<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let activity_id = &record.data["Event"]["System"]["Correlation"]["#attributes"]["ActivityID"];
        Ok(Self(activity_id))
    }
}

impl<'a> Display for ActivityId<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str().unwrap())
    }
}

const GUID_MAX_LENGTH: usize = "F4202F00-1781-44ED-99B9-1FAA35640000".len();
impl<'a> EvtxFieldView for ActivityId<'a> {
    fn maximum_display_length(&self) -> usize {
        GUID_MAX_LENGTH
    }

    fn value_with_padding(&self) -> String {
        self.0.as_str().unwrap_or("                                    ").to_owned()
    }
}

impl<'a> ActivityId<'a> {
    pub fn value(&self) -> &Value {
        &self.0
    }
}