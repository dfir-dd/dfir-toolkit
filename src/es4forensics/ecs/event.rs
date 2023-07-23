use std::collections::HashMap;

use duplicate::duplicate_item;
use serde::Serialize;
use serde_json::Value;

use super::{CustomizableField, ecs_object::EcsObject};

#[derive(Serialize)]
pub enum Kind {
    Alert,
    Enrichment,
    Event,
    Metric,
    State,
    PipelineError,
    Signal,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Event
    }
}

#[derive(Serialize)]
pub enum Category {
    Authentication,
    Configuration,
    Database,
    Driver,
    Email,
    File,
    Host,
    Iam,
    IntrusionDetection,
    Malware,
    Network,
    Package,
    Process,
    Registry,
    Session,
    Threat,
    Web,
}

#[derive(Serialize)]
pub enum Type {
    Access,
    Admin,
    Allowed,
    Change,
    Connection,
    Creation,
    Deletion,
    Denied,
    End,
    Error,
    Group,
    Indicator,
    Info,
    Installation,
    Protocol,
    Start,
    User,
}

#[derive(Serialize)]
pub enum Outcome {
    Failure,
    Success,
    Unknown,
}

#[derive(Default, Serialize)]
pub struct Event<'a> {
    event_kind: Kind,
    event_category: Option<Category>,
    event_type: Option<Type>,
    event_outcome: Option<Outcome>,
    code: Option<u64>,
    activity: Option<String>,
    sequence: Option<String>,
    module: Option<String>,
    provider: Option<String>,
    severity: Option<u8>,
    custom_data: HashMap<&'a String, &'a Value>,
}

impl<'a> Event<'a> {
    pub fn with_kind(mut self, ts: Kind) -> Self {
        self.event_kind = ts;
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_category ] [ event_category ] [ Category ];
      [ with_type ]     [ event_type ]     [ Type ];
      [ with_outcome ]  [ event_outcome ]  [ Outcome ];
      [ with_code ]     [ code ]           [ u64 ];
      [ with_sequence ] [ sequence ]       [ String ];
      [ with_module ]   [ module ]         [ String ];
      [ with_provider ] [ provider ]       [ String ];
      [ with_severity ] [ severity ]       [ u8 ];
   )]
    pub fn method(mut self, ts: ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}

impl<'a> EcsObject for Event<'a> {
    fn object_key(&self) -> &'static str {
        "event"
    }
}

impl<'a> CustomizableField<'a> for Event<'a> {
    fn with_custom_data(
        mut self,
        custom_data: &HashMap<&'a String, &'a serde_json::Value>,
    ) -> Self {
        self.custom_data.extend(custom_data);
        self
    }
}
