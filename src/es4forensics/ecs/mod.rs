mod event;
mod host;
pub mod log;
mod file;
mod ecs_builder;
mod ecs_object;
mod timeline_object;
pub mod objects;
pub use ecs_builder::*;
pub use event::*;
pub use host::*;
pub use file::*;
pub use timeline_object::TimelineObject;

use std::collections::HashMap;

use serde_json::Value;

pub trait ECSFields {
    fn into(&self) -> Value;
}

pub trait CustomizableField<'a> {
    fn with_custom_data(self, custom_data: &HashMap<&'a String, &'a Value>) -> Self;
}
