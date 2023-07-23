use serde::Serialize;
use serde_json::Value;

use super::ecs_object::EcsObject;

#[derive(Serialize)]
pub struct Host {
    name: Value
}

impl EcsObject for Host {
    fn object_key(&self) -> &'static str {
        "host"
    }
}

impl From<&Value> for Host {
    fn from(val: &Value) -> Self {
        Self {
            name: val.clone()
        }
    }
}
