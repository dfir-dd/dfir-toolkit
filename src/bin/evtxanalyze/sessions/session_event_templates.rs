use eventdata::{
    EventId, EventProvider, NoSessionId, SessionId, SessionIdGenerator, SessionNameInActivityId,
    SessionNameInLogonId,
};
use evtx::SerializedEvtxRecord;
use serde_json::Value;
use sessionevent_derive::SessionEvent;

use eventdata::SessionEventInfo;

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-RemoteConnectionManager",
    event_id = 1149,
    description = "User authentication succeeded",
    session_id = "ActivityId",
    username_path = "Event/UserData/EventXML/Param1",
    domain_path = "Event/UserData/EventXML/Param2",
    client_address_path = "Event/UserData/EventXML/Param3"
)]
pub struct TSRCMUserAuthenticationSucceeded {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 21,
    description = "Remote Desktop Services: Session logon succeeded",
    session_id = "ActivityId",
    username_path = "Event/UserData/EventXML/User",
    client_address_path = "Event/UserData/EventXML/Address"
)]

pub struct TSLCMSessionLogonSucceeded {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 22,
    description = "Remote Desktop Services: Shell start notification received",
    session_id = "ActivityId",
    username_path = "Event/UserData/EventXML/User",
    client_address_path = "Event/UserData/EventXML/Address"
)]
pub struct TSLCMShellStartNotificationReceived {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 23,
    description = "Remote Desktop Services: Session logoff succeeded",
    session_id = "ActivityId",
    username_path = "Event/UserData/EventXML/User",
    client_address_path = "Event/UserData/EventXML/Address"
)]
pub struct TSLCMSessionLogoffSucceeded {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 24,
    description = "Remote Desktop Services: Session has been disconnected",
    session_id = "ActivityId",
    username_path = "Event/UserData/EventXML/User",
    client_address_path = "Event/UserData/EventXML/Address"
)]
pub struct TSLCMSessionHasBeenDisconnected {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 25,
    description = "Remote Desktop Services: Session reconnection succeeded",
    session_id = "ActivityId"
)]
pub struct TSLCMSessionReconnectionSucceeded {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 39,
    description = "Session <X> has been disconnected by session <Y>",
    session_id = "ActivityId"
)]
pub struct TSLCMSessionXHasBeenDisconnectedBySessionY {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-TerminalServices-LocalSessionManager",
    event_id = 40,
    description = "Session <X> has been disconnected, reason code <Z>",
    session_id = "ActivityId"
)]
pub struct TSLCMSessionXHasBeenDisconnectedReasonCodeZ {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4624,
    description = "An account was successfully logged on",
    session_id = "LogonId",
    username_path = "Event/EventData/TargetUserName",
    client_hostname_path = "Event/EventData/WorkstationName",
    domain_path =  "Event/EventData/TargetDomainName",
)]
pub struct SecuritySuccessfulLogin {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4625,
    description = "An account failed to log on",
    session_id = "None",
    username_path = "Event/EventData/TargetUserName",
    client_hostname_path = "Event/EventData/WorkstationName",
    domain_path =  "Event/EventData/TargetDomainName",
)]
pub struct SecurityFailedLogin {}


#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4627,
    description = "Group membership information.",
    session_id = "LogonId",
    username_path = "Event/EventData/TargetUserName",
    domain_path =  "Event/EventData/TargetDomainName",
)]
pub struct SecurityGroupMembership {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4634,
    description = "An account was successfully logged off",
    session_id = "LogonId",
    username_path = "Event/EventData/TargetUserName"
)]
pub struct SecuritySuccessfulLogoff {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4647,
    description = "User initiated logoff",
    session_id = "LogonId",
    username_path = "Event/EventData/TargetUserName"
)]
pub struct SecurityUserInitiatedLogoff {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4778,
    description = "A session was reconnected to a Window Station",
    session_id = "LogonId",
    username_path = "Event/EventData/AccountName",
    domain_path =  "Event/EventData/AccountDomain",
    client_address_path = "Event/EventData/ClientAddress",
    client_hostname_path = "Event/EventData/ClientName"
)]
pub struct SecuritySessionWasReconnected {}

#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-Security-Auditing",
    event_id = 4779,
    description = "A session was disconnected from a Window Station.",
    session_id = "LogonId",
    username_path = "Event/EventData/AccountName",
    domain_path =  "Event/EventData/AccountDomain",
    client_address_path = "Event/EventData/ClientAddress",
    client_hostname_path = "Event/EventData/ClientName"
)]
pub struct SecuritySessionWasDisconnected {}


#[derive(SessionEvent, Default)]
#[event_data(
    provider = "Microsoft-Windows-RemoteDesktopServices-RdpCoreTS",
    event_id = 131,
    description = "The server accepted a new connection",
    session_id = "ActivityId",
    client_address_path = "Event/EventData/ClientIP",
)]
pub struct RdpAcceptedConnection {}
