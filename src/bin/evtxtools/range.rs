use std::fmt::Display;

use crate::event_id::EventId;

#[derive(PartialEq, Eq)]
pub struct Range {
    events: Vec<EventId>
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.begin().timestamp().cmp(other.begin().timestamp())
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.begin().timestamp().cmp(other.begin().timestamp()))
    }
}

impl Range {
    #[allow(dead_code)]
    pub fn from(begin: EventId) -> Self {
        Self {
            events: vec![begin]
        }
    }

    pub fn begin(&self) -> &EventId {
        &self.events[0]
    }

    pub fn end(&self) -> &EventId {
        &self.events[self.events.len()-1]
    }

    #[allow(dead_code)]
    pub fn add_event(&mut self, end: EventId) {
        assert!(self.can_contain(&end));
        self.events.push(end);
    }

    #[allow(dead_code)]
    pub fn can_contain(&self, id: &EventId) -> bool {
        id.follows(self.end())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[allow(dead_code)]
    pub fn events(&self) -> std::slice::Iter<'_, EventId> {
        self.events.iter()
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({} - {})",
            self.begin().timestamp().format("%FT%T"), 
            self.end().timestamp().format("%FT%T"), 
            self.begin().event_record_id(), 
            self.end().event_record_id())
    }
}