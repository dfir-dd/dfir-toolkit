use chrono::{DateTime, Utc, Duration};
use evtx::SerializedEvtxRecord;


#[derive(Eq, Hash, PartialEq, Clone)]
pub struct EventId {
    timestamp: DateTime<Utc>,
    event_record_id: u64,
    allowed_bias: Duration,
}

impl Default for EventId {
    fn default() -> Self {
        Self {
            timestamp: Default::default(),
            event_record_id: u64::default(),
            allowed_bias: Duration::seconds(10)
        }
    }
}

impl Ord for EventId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.event_record_id.cmp(&other.event_record_id)

        /* match self.timestamp.cmp(&other.timestamp) {
            std::cmp::Ordering::Equal => self.event_record_id.cmp(&other.event_record_id),
            ord => ord
        } */
    }
}

impl PartialOrd for EventId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl EventId {
    #[allow(dead_code)]
    pub fn from<T>(record: &SerializedEvtxRecord<T>) -> Self {
        Self {
            timestamp: record.timestamp,
            event_record_id: record.event_record_id,
            allowed_bias: Duration::seconds(10)
        }
    }

    #[allow(dead_code)]
    pub fn follows(&self, other: &Self) -> bool {
        /*self.timestamp + self.allowed_bias >= other.timestamp && */ self.event_record_id == other.event_record_id + 1
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn event_record_id(&self) -> u64 {
        self.event_record_id
    }
}