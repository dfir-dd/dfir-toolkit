use std::collections::{HashMap, HashSet};

use anyhow::bail;
use duplicate::duplicate_item;
use serde_json::{json, Value};

use crate::es4forensics::ecs::ecs_object::EcsObject;
use crate::es4forensics::ecs::{log::Log, Event, File, Host};
use crate::es4forensics::timestamp::Timestamp;

pub struct EcsBuilder {
    ts: Timestamp,
    message: String,
    //labels: HashMap<String, String>,
    tags: HashSet<String>,
    contents: HashMap<&'static str, Value>,
}

impl EcsBuilder {
    pub fn new(message: String, ts: Timestamp) -> Self {
        Self {
            ts,
            message,
            tags: HashSet::default(),
            contents: HashMap::default(),
        }
    }

    pub fn with_additional_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_owned());
        self
    }

    #[duplicate_item(
        method       ret_type;
    [ with_event ] [ Event<'_> ];
    [ with_host ]  [ Host ];
    [ with_log ]   [ Log ];
    [ with_file ]  [ File ];
    )]
    pub fn method(mut self, ts: ret_type) -> anyhow::Result<Self> {
        if self.contents.contains_key(ts.object_key()) {
            bail!("unambigious key: '{}'", ts.object_key());
        }
        self.contents.insert(ts.object_key(), json!(ts));
        Ok(self)
    }
}

impl From<EcsBuilder> for (Timestamp, Value) {
    fn from(val: EcsBuilder) -> (Timestamp, Value) {
        let mut m = HashMap::from([
            (
                "@timestamp",
                Value::Number(val.ts.timestamp_millis().into()),
            ),
            ("ecs", json!({"version": "8.4"})),
            ("message", json!(val.message)),
        ]);

        if !val.tags.is_empty() {
            m.insert("tags", json!(val.tags));
        }

        for (key, value) in val.contents.into_iter() {
            m.insert(key, value);
        }
        (val.ts, json!(m))
    }
}
