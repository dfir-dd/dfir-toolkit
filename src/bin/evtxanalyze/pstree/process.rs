use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt::Display,
    hash::Hash,
    rc::Weak,
};

use anyhow::bail;
use chrono::{DateTime, Utc};
use evtx::SerializedEvtxRecord;
use serde::Serialize;
use serde_json::{json, Value};

pub(crate) struct Process {
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) event_record_id: u64,
    pub(crate) subject_user_sid: String,
    pub(crate) subject_user_name: String,
    pub(crate) subject_domain_name: String,
    pub(crate) subject_logon_id: String,
    pub(crate) new_process_id: u64,
    pub(crate) new_process_name: String,
    pub(crate) token_elevation_type: String,
    pub(crate) process_id: u64,
    pub(crate) command_line: String,
    pub(crate) target_user_sid: String,
    pub(crate) target_user_name: String,
    pub(crate) target_domain_name: String,
    pub(crate) target_logon_id: String,
    pub(crate) parent_process_name: Option<String>,
    pub(crate) mandatory_label: Option<String>,
    pub(crate) children: BTreeMap<DateTime<Utc>, Weak<RefCell<Self>>>,
    pub(crate) is_root: bool,
}

#[derive(Serialize, Clone)]
pub(crate) struct ProcessTableEntry {
    pub(crate) subject_user_sid: String,
    pub(crate) subject_user_name: String,
    pub(crate) subject_domain_name: String,
    pub(crate) subject_logon_id: String,
    pub(crate) new_process_id: u64,
    pub(crate) new_process_name: String,
    pub(crate) token_elevation_type: String,
    pub(crate) process_id: u64,
    pub(crate) command_line: String,
    pub(crate) target_user_sid: String,
    pub(crate) target_user_name: String,
    pub(crate) target_domain_name: String,
    pub(crate) target_logon_id: String,
    pub(crate) parent_process_name: Option<String>,
}

impl From<&Process> for ProcessTableEntry {
    fn from(value: &Process) -> Self {
        Self {
            subject_user_sid: value.subject_user_sid.clone(),
            subject_user_name: value.subject_user_name.clone(),
            subject_domain_name: value.subject_domain_name.clone(),
            subject_logon_id: value.subject_logon_id.clone(),
            new_process_id: value.new_process_id,
            new_process_name: value.new_process_name.clone(),
            token_elevation_type: value.token_elevation_type.clone(),
            process_id: value.process_id,
            command_line: value.command_line.clone(),
            target_user_sid: value.target_user_sid.clone(),
            target_user_name: value.target_user_name.clone(),
            target_domain_name: value.target_domain_name.clone(),
            target_logon_id: value.target_logon_id.clone(),
            parent_process_name: value.parent_process_name.clone(),
        }
    }
}

impl Hash for ProcessTableEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.process_id.hash(state);
        self.command_line.hash(state);
        self.parent_process_name.hash(state);
    }
}

impl Eq for ProcessTableEntry {}
impl PartialEq for ProcessTableEntry {
    fn eq(&self, other: &Self) -> bool {
        self.process_id == other.process_id
            && self.command_line == other.command_line
            && self.parent_process_name == other.parent_process_name
    }
}

impl From<&Process> for Value {
    fn from(process: &Process) -> Self {
        let children: BTreeMap<_, _> = process
            .children
            .values()
            .filter_map(|x| x.upgrade())
            .map(|p| {
                let p: &Process = &p.borrow();
                let v: Value = p.into();
                (p.timestamp, v)
            })
            .collect();
        let mut result: HashMap<_, _> = vec![
            ("timestamp".to_owned(), json!(process.timestamp)),
            ("event_record_id".to_owned(), json!(process.event_record_id)),
            ("SubjectUserSid".to_owned(), json!(process.subject_user_sid)),
            (
                "SubjectUserName".to_owned(),
                json!(process.subject_user_name),
            ),
            (
                "SubjectDomainName".to_owned(),
                json!(process.subject_domain_name),
            ),
            ("SubjectLogonId".to_owned(), json!(process.subject_logon_id)),
            ("NewProcessId".to_owned(), json!(process.new_process_id)),
            ("NewProcessName".to_owned(), json!(process.new_process_name)),
            (
                "TokenElevationType".to_owned(),
                json!(process.token_elevation_type),
            ),
            ("ProcessId".to_owned(), json!(process.process_id)),
            ("CommandLine".to_owned(), json!(process.command_line)),
            ("TargetUserSid".to_owned(), json!(process.target_user_sid)),
            ("TargetUserName".to_owned(), json!(process.target_user_name)),
            (
                "TargetDomainName".to_owned(),
                json!(process.target_domain_name),
            ),
            ("TargetLogonId".to_owned(), json!(process.target_logon_id)),
            (
                "ParentProcessName".to_owned(),
                json!(process.parent_process_name),
            ),
            ("MandatoryLabel".to_owned(), json!(process.mandatory_label)),
        ]
        .into_iter()
        .collect();

        result.extend(
            children
                .into_iter()
                .map(|(k, v)| (k.to_rfc3339_opts(chrono::SecondsFormat::Secs, true), v)),
        );

        json!(result)
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}` (`0x{:04x}`, created *`{}`*, user is `{}`)",
            self.new_process_name,
            self.new_process_id,
            self.timestamp
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            self.target_user_name
        )
    }
}

macro_rules! from_json {
    ($value: ident, $( $att:expr ),+ ) => {
        {
            let mut value = $value;
            $(
                value = value.get($att).ok_or(anyhow::anyhow!("missing '{}' key in {}", $att, value))?;
            )+
            value
        }
    };
}

macro_rules! from_json_or_null {
    ($value: ident, $( $att:expr ),+) => {
        {
            let mut value = $value;
            $(
                value = value.get($att).or(Some(&Value::Null)).unwrap();
            )+
            value
        }
    };
}

fn u64_from_value(value: &Value) -> anyhow::Result<u64> {
    if let Some(v) = value.as_u64() {
        Ok(v)
    } else {
        bail!("Value '{value}' is no u64")
    }
}
fn u64_from_hex_value(value: &Value) -> anyhow::Result<u64> {
    if let Some(v) = value.as_str() {
        Ok(u64::from_str_radix(v.trim_start_matches("0x"), 16)?)
    } else {
        bail!("Value '{value}' is no string")
    }
}

impl Process {
    pub fn try_from(record: SerializedEvtxRecord<Value>) -> anyhow::Result<Option<Self>> {
        let value = &record.data;
        let event = from_json!(value, "Event");
        let system = from_json!(event, "System");
        let event_id = u64_from_value({
            let event_id = from_json!(system, "EventID");
            match event_id.get("#text") {
                Some(eid) => eid,
                None => event_id,
            }
        })?;

        if event_id != 4688 {
            return Ok(None);
        }

        let event_data = from_json!(event, "EventData");

        let subject_user_sid = from_json!(event_data, "SubjectUserSid")
            .as_str()
            .unwrap()
            .into();
        let subject_user_name = from_json!(event_data, "SubjectUserName")
            .as_str()
            .unwrap()
            .into();
        let subject_domain_name = from_json!(event_data, "SubjectDomainName")
            .as_str()
            .unwrap()
            .into();
        let subject_logon_id = from_json!(event_data, "SubjectLogonId")
            .as_str()
            .unwrap()
            .into();
        let new_process_id = u64_from_hex_value(from_json!(event_data, "NewProcessId"))?;
        let new_process_name = from_json!(event_data, "NewProcessName")
            .as_str()
            .unwrap()
            .into();
        let token_elevation_type = from_json!(event_data, "TokenElevationType")
            .as_str()
            .unwrap()
            .into();
        let process_id = u64_from_hex_value(from_json!(event_data, "ProcessId"))?;
        let command_line = from_json!(event_data, "CommandLine")
            .as_str()
            .unwrap()
            .into();
        let target_user_sid = from_json!(event_data, "TargetUserSid")
            .as_str()
            .unwrap()
            .into();
        let target_user_name = from_json!(event_data, "TargetUserName")
            .as_str()
            .unwrap()
            .into();
        let target_domain_name = from_json!(event_data, "TargetDomainName")
            .as_str()
            .unwrap()
            .into();
        let target_logon_id = from_json!(event_data, "TargetLogonId")
            .as_str()
            .unwrap()
            .into();
        let parent_process_name = from_json_or_null!(event_data, "ParentProcessName")
            .as_str()
            .map(|s| s.to_owned());
        let mandatory_label = from_json_or_null!(event_data, "MandatoryLabel")
            .as_str()
            .map(|s| s.to_owned());

        Ok(Some(Self {
            timestamp: record.timestamp,
            event_record_id: record.event_record_id,
            subject_user_sid,
            subject_user_name,
            subject_domain_name,
            subject_logon_id,
            new_process_id,
            new_process_name,
            token_elevation_type,
            process_id,
            command_line,
            target_user_sid,
            target_user_name,
            target_domain_name,
            target_logon_id,
            parent_process_name,
            mandatory_label,
            children: Default::default(),
            is_root: true,
        }))
    }
}
