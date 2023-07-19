use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use es4forensics::{objects::WindowsEvent, TimelineObject};
use evtx::SerializedEvtxRecord;
use getset::{Getters, Setters};
use serde::Serialize;
use serde_json::{json, Value};

use crate::macros::from_json;

#[derive(Serialize, Getters, Setters)]
pub(crate) struct BfData<'a> {
    event_record_id: u64,
    timestamp: DateTime<Utc>,
    #[getset(get)]
    event_id: &'a Value,
    level: &'a Value,
    computer: &'a Value,
    provider_name: &'a Value,
    channel_name: &'a Value,
    activity_id: Option<&'a Value>,
    custom_data: HashMap<&'a String, &'a Value>,

    #[serde(skip)]
    #[getset(set="pub (crate)")]
    enable_json_output: bool,
}

impl<'a> BfData<'a> {

    pub(crate) fn try_into_mactime(&self) -> Result<String> {
        let bf_line = Bodyfile3Line::new()
            .with_mtime(self.timestamp.timestamp())
            .with_owned_name(json!(self).to_string());
        Ok(bf_line.to_string())
    }

    pub(crate) fn try_into_json(&self) -> Result<String> {
        let event: WindowsEvent = self.try_into()?;
        let values: Vec<Value> = event.into_values().collect();
        if values.len() != 1 {
            bail!("this event resolved to an invalid number of JSON documents");
        }
        serde_json::to_string(&values[0]).map_err(|why| anyhow!(why))
    }
}

impl<'a> TryFrom<&BfData<'a>> for String {
    type Error = anyhow::Error;

    fn try_from(value: &BfData<'a>) -> Result<Self, Self::Error> {
        if value.enable_json_output {
            value.try_into_json()
        } else {
            value.try_into_mactime()
        }
    }
}

impl<'a> TryFrom<BfData<'a>> for String {
    type Error = anyhow::Error;

    fn try_from(value: BfData<'a>) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

impl<'a> TryFrom<&'a SerializedEvtxRecord<Value>> for BfData<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let value = &record.data;
        let event = from_json!(value, "Event");
        let system = from_json!(event, "System");
        let event_id = {
            let event_id = from_json!(system, "EventID");
            match event_id.get("#text") {
                Some(eid) => eid,
                None => event_id,
            }
        };

        let level = from_json!(system, "Level");
        let computer = from_json!(system, "Computer");
        let provider_name = from_json!(system, "Provider", "#attributes", "Name");
        let channel_name = from_json!(system, "Channel");

        let activity_id = system
            .get("Correlation")
            .and_then(|c| c.get("#attributes"))
            .and_then(|c| c.get("ActivityId"));

        let mut custom_data = HashMap::new();
        if let Value::Object(contents) = event {
            for (key, value) in contents.iter() {
                if key != "System" && key != "#attributes" {
                    custom_data.insert(key, value);
                }
            }
        }

        Ok(Self {
            event_record_id: record.event_record_id,
            timestamp: record.timestamp,
            event_id,
            level,
            computer,
            provider_name,
            channel_name,
            activity_id,
            custom_data,
            enable_json_output: false,
        })
    }
}

impl<'a> TryFrom<&BfData<'a>> for WindowsEvent<'a> {
    type Error = anyhow::Error;

    fn try_from(bfdata: &BfData<'a>) -> Result<Self, Self::Error> {
        let event_id = match bfdata.event_id.as_u64() {
            Some(id) => id,
            _ => bail!("invalid event id: {:?}", bfdata.event_id),
        };
        Ok(WindowsEvent::new(
            bfdata.event_record_id,
            bfdata.timestamp,
            event_id,
            bfdata.level.try_into()?,
            bfdata.computer,
            bfdata.provider_name,
            bfdata.channel_name,
            bfdata.activity_id,
            bfdata.custom_data.clone(),
        ))
    }
}
