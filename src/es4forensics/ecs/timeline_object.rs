use serde_json::Value;

use crate::es4forensics::timestamp::Timestamp;

use super::ecs_builder::EcsBuilder;

pub trait TimelineObject: IntoIterator<Item = anyhow::Result<EcsBuilder>> {
    fn into_values(self) -> Box<dyn Iterator<Item = Value>>
    where
        Self: Sized,
        <Self as std::iter::IntoIterator>::IntoIter: 'static,
    {
        let res = self.into_iter().filter_map(|b| b.ok()).map(|b| {
            let (_, v) = b.into();
            v
        });
        Box::new(res)
    }

    fn into_tuples(self) -> Box<dyn Iterator<Item = (Timestamp, Value)>>
    where
        Self: Sized,
        <Self as std::iter::IntoIterator>::IntoIter: 'static,
    {
        let res = self
            .into_iter()
            .filter_map(|b| b.ok())
            .map(EcsBuilder::into);
        Box::new(res)
    }
}
