use chrono::{DateTime, Utc};
use eventdata::{EventId, EventLevel};
use serde::Serialize;

use super::SessionEvent;

#[derive(Serialize)]
pub struct EventAsCsv {
    level: Option<EventLevel>,
    timestamp: DateTime<Utc>,
    event_id: EventId,
    description: String,

    pub domain: Option<String>,

    pub username: Option<String>,

    pub client_address: Option<String>,
    pub client_name: Option<String>,

    pub server_address: Option<String>,
    pub server_name: Option<String>,
}

impl From<&SessionEvent> for EventAsCsv {
    fn from(value: &SessionEvent) -> Self {
        let timestamp = value.record().timestamp;
        let event_id = value.event_type().event_id();
        let description = value.event_type().description().to_owned();

        Self {
            level: EventLevel::try_from(value.record()).ok(),
            timestamp,
            event_id,
            description,
            domain: value.event_type().domain(value.record()),
            username: value.event_type().username(value.record()),
            client_address: value.event_type().client_address(value.record()),
            client_name: value.event_type().client_hostname(value.record()),
            server_address: value.event_type().server_address(value.record()),
            server_name: value.event_type().server_hostname(value.record()),
        }
    }
}
