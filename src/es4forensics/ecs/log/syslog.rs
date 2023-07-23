use serde::Serialize;

use super::Severity;

#[derive(Serialize, Default)]
pub struct Syslog {
    severity: Severity
}

impl Syslog {
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }
}