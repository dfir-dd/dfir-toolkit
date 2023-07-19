use std::fmt::Display;

use chrono::{DateTime, Utc};

use super::Process;

#[derive(Eq, PartialEq, Hash, Clone)]
pub (crate) struct UniquePid {
    timestamp: DateTime<Utc>,
    pid: u64
}

impl From<&Process> for UniquePid {
    fn from(process: &Process) -> Self {
        Self {
            timestamp: process.timestamp,
            pid: process.new_process_id
        }
    }
}

impl Display for UniquePid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PID '{}' started at {}", self.pid, self.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
    }
}

impl UniquePid {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

impl Ord for UniquePid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
impl PartialOrd for UniquePid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.pid != other.pid {
            None
        } else {
            Some(self.timestamp.cmp(&other.timestamp))
        }
    }
}