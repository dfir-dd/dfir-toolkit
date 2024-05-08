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
    value: Option<String>,
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
//#[serde(rename_all = "PascalCase")]
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

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::Event;

    #[test]
    fn test_attributes() {
        let s = r#"<?xml version="1.0" encoding="utf-8"?>
        <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
          <System>
            <Provider Name="Microsoft-Windows-WinRM" Guid="A7975C8F-AC13-49F1-87DA-5A984A4AB417">
            </Provider>
            <EventID>209</EventID>
            <Version>0</Version>
            <Level>4</Level>
            <Task>11</Task>
            <Opcode>0</Opcode>
            <Keywords>0x4000000000000004</Keywords>
            <TimeCreated SystemTime="2021-09-12T02:01:47.359872Z">
            </TimeCreated>
            <EventRecordID>7476</EventRecordID>
            <Correlation ActivityID="1A201BFC-A77A-0000-231C-201A7AA7D701">
            </Correlation>
            <Execution ProcessID="328" ThreadID="2884">
            </Execution>
            <Channel>Microsoft-Windows-WinRM/Operational</Channel>
            <Computer>DC1.sample.de</Computer>
            <Security UserID="S-1-5-20">
            </Security>
          </System>
          <EventData>
          </EventData>
        </Event>
        "#;
        let e: Event = from_str(s).unwrap();
        assert_eq!(e.system().security().user_id().as_ref().unwrap(), "S-1-5-20");  
    }
}