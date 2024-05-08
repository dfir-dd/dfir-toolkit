use getset::Getters;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct Event {
    system: System,
    event_data: Option<EventData>,
    rendering_info: Option<RenderingInfo>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct System {
    //provider: Provider,
    EventID: String,
    version: String,
    level: EventLevel,
    task: String,
    opcode: String,
    keywords: String,
    time_created: TimeCreated,
    EventRecordID: String,
    channel: String,
    computer: String,
    security: Security,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct TimeCreated {
    #[serde(rename = "@SystemTime")]
    system_time: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct EventData {
    data: Option<Vec<Data>>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct Data {
    #[serde(rename = "$value")]
    value: String,
    #[serde(rename = "@Name")]
    name: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct RenderingInfo {
    #[serde(rename = "@Culture")]
    culture: String,
    #[serde(rename = "$value")]
    message: String,
}
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "PascalCase")]
#[getset(get="pub")]
#[allow(unused,non_snake_case)]
pub struct Security {
    #[serde(rename = "@UserID")]
    user_id: Option<String>,
}

/// https://learn.microsoft.com/en-us/dotnet/api/system.diagnostics.tracing.eventlevel?view=net-8.0
#[derive(Debug, Deserialize_repr)]
#[serde(rename_all = "PascalCase")]
#[allow(unused,non_snake_case)]
#[repr(u8)]
pub enum EventLevel {
    LogAlways = 0,
    Critical = 1,
    Error = 2,
    Warning = 3,
    Information = 4,    
}

impl ToString for EventLevel {
    fn to_string(&self) -> String {
        match self {
            EventLevel::LogAlways => " ",
            EventLevel::Critical => "🔥",
            EventLevel::Error => "🛑",
            EventLevel::Warning => "⚠",
            EventLevel::Information => "ℹ",
        }.into()
    }
}
