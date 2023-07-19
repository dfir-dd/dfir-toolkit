use evtx::SerializedEvtxRecord;
use serde_json::Value;

use crate::{SessionId, EventId, EventProvider};

pub trait SessionEventInfo {
    fn event_id(&self) -> EventId;
    fn description(&self) -> &'static str;
    fn provider(&self) -> EventProvider;
    fn generate_id(&self, record: &SerializedEvtxRecord<Value>) -> SessionId;
    fn username(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;
    fn domain(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;
    fn client_address(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;
    fn client_hostname(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;
    fn server_address(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;
    fn server_hostname(&self, record: &SerializedEvtxRecord<Value>) -> Option<String>;

}