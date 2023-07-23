use anyhow::bail;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::Serialize;
use serde_json::Value;

/// Source: <https://learn.microsoft.com/de-de/dotnet/api/system.diagnostics.tracing.eventlevel?view=net-6.0>
#[derive(strum_macros::Display, FromPrimitive, ToPrimitive, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum EventLevel {
    LogAlways = 0,
    Critical = 1,
    Error = 2,
    Warning = 3,
    Information = 4,
    Verbose = 5,
}

impl TryFrom<&Value> for EventLevel {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, anyhow::Error> {
        match value.as_u64() {
            None => bail!("unable to convert '{value}' into u8"),
            Some(n) => match Self::from_u64(n) {
                None => bail!("invalid numeric value: '{n}'"),
                Some(me) => Ok(me)
            }
        }
    }
}