use std::sync::Mutex;

use crate::ActivityId;
use evtx::SerializedEvtxRecord;
use serde::Serialize;
use serde_json::Value;

#[derive(PartialEq, Eq, PartialOrd, Hash, Ord, Clone, Debug, Serialize)]
pub enum SessionId {
    ActivityId(String),
    SessionName(String),
    LogonId(String),
    SessionId(String),
    None(u64),
}

pub trait SessionIdGenerator {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId;
}

static NO_SESSION_ID_MUTEX: Mutex<u64> = Mutex::new(0);
pub struct NoSessionId {}
impl SessionIdGenerator for NoSessionId {
    fn session_id_of(_: &SerializedEvtxRecord<Value>) -> SessionId {
        let mut id_mutex = NO_SESSION_ID_MUTEX.lock().unwrap();
        let id = *id_mutex;
        *id_mutex = id + 1;
        SessionId::None(id)
    }
}

pub struct SessionNameInEventData {}
impl SessionIdGenerator for SessionNameInEventData {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::SessionName(
            record.data["Event"]["EventData"]["SessionName"]
                .as_str()
                .expect("missing SessionName in event")
                .into(),
        )
    }
}

pub struct SessionNameInActivityId {}
impl SessionIdGenerator for SessionNameInActivityId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        let activity_id = ActivityId::try_from(record).expect("missing activity id in event");

        match activity_id.value().as_str() {
            None => {
                SessionId::ActivityId("".into())
            }
            Some(activity_id) => {
                SessionId::ActivityId(activity_id.into())
            }
        }
    }
}

pub struct SessionNameInTargetLogonId {}
impl SessionIdGenerator for SessionNameInTargetLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["EventData"]["SessionName"]
                .as_str()
                .expect("missing TargetLogonId in event")
                .into(),
        )
    }
}

pub struct SessionNameInSubjectLogonId {}
impl SessionIdGenerator for SessionNameInSubjectLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["EventData"]["SubjectLogonId"]
                .as_str()
                .expect("missing SubjectLogonId in event")
                .into(),
        )
    }
}

pub struct SessionNameInLogonId {}
impl SessionIdGenerator for SessionNameInLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        if let Some(children) = record.data["Event"]["EventData"].as_object() {
            for child in children {
                if child.0.to_lowercase() == "targetlogonid" {
                    return SessionId::LogonId(child.1.as_str().unwrap().to_owned())
                }if child.0.to_lowercase() == "logonid" {
                    return SessionId::LogonId(child.1.as_str().unwrap().to_owned())
                }
            }
        }
        panic!("missing LogonId in event: {event}", event = record.data);
    }
}

pub struct SessionIdInUserData {}
impl SessionIdGenerator for SessionIdInUserData {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["UserData"]["EventXML"]["SessionID"]
                .as_str()
                .expect("missing SessionID in event")
                .into(),
        )
    }
}
