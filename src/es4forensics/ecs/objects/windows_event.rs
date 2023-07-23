use std::collections::HashMap;

use crate::es4forensics::{
    ecs::{ecs_builder::EcsBuilder, timeline_object::TimelineObject, *},
    ecs::log::{EventLevel, Log, Severity, Syslog},
};
use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use serde_json::Value;

pub struct WindowsEvent<'a> {
    event_record_id: u64,

    timestamp: DateTime<Utc>,

    event_id: u64,
    level: EventLevel,
    computer: &'a Value,

    provider_name: &'a Value,
    channel_name: &'a Value,
    _activity_id: Option<&'a Value>,
    custom_data: HashMap<&'a String, &'a Value>,
}

impl<'a> WindowsEvent<'a> {
    #[allow(clippy::too_many_arguments)]
    #[allow(dead_code)]
    pub fn new(
        event_record_id: u64,
        timestamp: DateTime<Utc>,
        event_id: u64,
        level: EventLevel,
        computer: &'a Value,
        provider_name: &'a Value,
        channel_name: &'a Value,
        _activity_id: Option<&'a Value>,
        custom_data: HashMap<&'a String, &'a Value>,
    ) -> Self {
        Self {
            event_record_id,
            timestamp,
            event_id,
            level,
            computer,
            provider_name,
            channel_name,
            _activity_id,
            custom_data,
        }
    }

    fn into_builder(self) -> anyhow::Result<EcsBuilder> {
        let event = Event::default()
            .with_kind(Kind::Event)
            .with_sequence(self.event_record_id.to_string())
            .with_code(self.event_id)
            .with_module(self.channel_name.as_str().unwrap().to_owned())
            .with_provider(self.provider_name.as_str().unwrap().to_owned())
            .with_severity(self.level.to_u8().unwrap())
            .with_custom_data(&self.custom_data);

        let host = Host::from(self.computer);

        let log =
            Log::default().with_syslog(Syslog::default().with_severity(Severity::from(self.level)));

        EcsBuilder::new(format!("{}: {}", self.channel_name, self.event_id), self.timestamp.into())
            .with_event(event)?
            .with_host(host)?
            .with_log(log)
    }
}

impl TimelineObject for WindowsEvent<'_> {}

impl IntoIterator for WindowsEvent<'_> {
    type Item = anyhow::Result<EcsBuilder>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self.into_builder()].into_iter()
    }
}
