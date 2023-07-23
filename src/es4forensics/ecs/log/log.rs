use serde::Serialize;

use crate::es4forensics::ecs::ecs_object::EcsObject;

use super::Syslog;

#[derive(Serialize, Default)]
pub struct Log {
    syslog: Option<Syslog>
}

impl Log {
    pub fn with_syslog(mut self, syslog: Syslog) -> Self {
        self.syslog = Some(syslog);
        self
    }
}

impl EcsObject for Log {
    fn object_key(&self) -> &'static str {
        "log"
    }
}