use serde::Serialize;

pub trait EcsObject: Serialize {
    fn object_key(&self) -> &'static str;
}