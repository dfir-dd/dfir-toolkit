use serde::Serialize;
use num_traits::ToPrimitive;
use std::string::ToString;

use super::EventLevel;

#[derive(Serialize)]
pub struct Severity {
    code: u8,
    name: String
}

impl From<EventLevel> for Severity {
    fn from(level: EventLevel) -> Self {
        Self {
            code: level.to_u8().unwrap(),
            name: level.to_string()
        }
    }
}

impl Default for Severity {
    fn default() -> Self {
        Self::from(EventLevel::Information)
    }
}