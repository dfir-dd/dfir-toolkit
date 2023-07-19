use std::fmt::Display;

use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::EvtxFieldView;

#[derive(PartialEq, Eq, Clone)]
pub struct RelatedActivityId<'a>(Option<&'a Value>);

impl<'a> TryFrom<&'a SerializedEvtxRecord<Value>> for RelatedActivityId<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let activity_id = &record.data["Event"]["System"]["Correlation"]["#attributes"]["RelatedActivityID"];
        if activity_id.is_null() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(activity_id)))
        }
    }
}

impl<'a> Display for RelatedActivityId<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            Some(v) => v.fmt(f),
            None => "None".fmt(f),
        }
    }
}

const GUID_MAX_LENGTH: usize = "F4202F00-1781-44ED-99B9-1FAA35640000".len();
impl<'a> EvtxFieldView for RelatedActivityId<'a> {
    fn maximum_display_length(&self) -> usize {
        GUID_MAX_LENGTH
    }

    fn value_with_padding(&self) -> String {
        match self.0 {
            Some(v) => v.to_string(),
            None => "                                    ".to_owned(),
        }
    }
}