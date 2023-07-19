use std::io::Write;

use eventdata::{
    EventId, EventProvider, SessionId
};
use evtx::SerializedEvtxRecord;
use serde_json::Value;

use eventdata::SessionEventInfo;
use super::EventAsCsv;
use super::session_event_templates::*;

use super::SessionEventError;

pub struct SessionEvent {
    event_type: Box<dyn SessionEventInfo>,
    record: SerializedEvtxRecord<serde_json::Value>,
    session_id: SessionId,
}

impl SessionEvent {
    fn new<I>(record: SerializedEvtxRecord<serde_json::Value>) -> Self
    where
        I: SessionEventInfo + Default + 'static,
    {
        let event_type = Box::<I>::default();
        let session_id = event_type.generate_id(&record);
        Self {
            event_type,
            record,
            session_id,
        }
    }

    pub fn event_type(&self) -> &dyn SessionEventInfo {
        self.event_type.as_ref()
    }

    pub fn record(&self) -> &SerializedEvtxRecord<Value> {
        &self.record
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn to_csv<W>(&self, writer: &mut csv::Writer<W>) -> csv::Result<()>
    where
        W: Write,
    {
        writer.serialize(&Into::<EventAsCsv>::into(self))
    }
}

impl TryFrom<SerializedEvtxRecord<serde_json::Value>> for SessionEvent {
    type Error = SessionEventError;

    fn try_from(record: SerializedEvtxRecord<serde_json::Value>) -> Result<Self, Self::Error> {
        let event_id = EventId::try_from(&record)?;
        let provider = EventProvider::try_from(&record)?;
        let event = match provider {
            EventProvider::TerminalServicesRemoteConnectionManager => match event_id.value() {
                1149 => Self::new::<TSRCMUserAuthenticationSucceeded>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            EventProvider::TerminalServicesLocalSessionManager => match event_id.value() {
                21 => Self::new::<TSLCMSessionLogonSucceeded>(record),
                22 => Self::new::<TSLCMShellStartNotificationReceived>(record),
                23 => Self::new::<TSLCMSessionLogoffSucceeded>(record),
                24 => Self::new::<TSLCMSessionHasBeenDisconnected>(record),
                25 => Self::new::<TSLCMSessionReconnectionSucceeded>(record),
                39 => Self::new::<TSLCMSessionXHasBeenDisconnectedBySessionY>(record),
                40 => Self::new::<TSLCMSessionXHasBeenDisconnectedReasonCodeZ>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            EventProvider::SecurityAuditing => match event_id.value() {
                4624 => Self::new::<SecuritySuccessfulLogin>(record),
                4625 => Self::new::<SecurityFailedLogin>(record),
                4627 => Self::new::<SecurityGroupMembership>(record),
                4634 => Self::new::<SecuritySuccessfulLogoff>(record),
                4647 => Self::new::<SecurityUserInitiatedLogoff>(record),
                4778 => Self::new::<SecuritySessionWasReconnected>(record),
                4779 => Self::new::<SecuritySessionWasDisconnected>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            EventProvider::RemoteDesktopServicesRdpCoreTS => match event_id.value() {
                131 => Self::new::<RdpAcceptedConnection>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            }
            _ => {
                log::warn!("unknown event provider: {provider}");
                return Err(SessionEventError::NoSessionEvent);
            }
        };

        assert_eq!(&event_id, &event.event_type.event_id());
        assert_eq!(&provider, &event.event_type.provider());

        Ok(event)
    }
}

impl Ord for SessionEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.record.timestamp.cmp(&other.record.timestamp)
    }
}

impl PartialOrd for SessionEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SessionEvent {}

impl PartialEq for SessionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.record.timestamp.eq(&other.record.timestamp)
    }
}
