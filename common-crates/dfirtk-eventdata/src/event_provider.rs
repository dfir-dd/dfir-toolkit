use std::fmt::Display;

use darling::FromMeta;
use evtx::SerializedEvtxRecord;
use quote::quote;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum EventProvider {
    TerminalServicesRemoteConnectionManager,
    TerminalServicesLocalSessionManager,
    RemoteDesktopServicesRdpCoreTS,
    SecurityAuditing,
    DesktopWindowManager,
    UnsupportedProvider,
}

impl Display for EventProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventProvider::TerminalServicesRemoteConnectionManager => {
                "Microsoft-Windows-Terminal-Services-RemoteConnectionManager"
            }
            EventProvider::TerminalServicesLocalSessionManager => {
                "Microsoft-Windows-TerminalServices-LocalSessionManager"
            }
            EventProvider::RemoteDesktopServicesRdpCoreTS => {
                "Microsoft-Windows-RemoteDesktopServices-RdpCoreTS"
            }
            EventProvider::SecurityAuditing => "Microsoft-Windows-Security-Auditing",
            EventProvider::DesktopWindowManager => "Desktop Window Manager",
            EventProvider::UnsupportedProvider => "UNSUPPORTED PROVIDER",
        }
        .fmt(f)
    }
}

impl TryFrom<&SerializedEvtxRecord<Value>> for EventProvider {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let provider_name = record.data["Event"]["System"]["Provider"]["#attributes"]["Name"]
            .as_str()
            .unwrap();
        Self::try_from(provider_name)
    }
}

impl TryFrom<&str> for EventProvider {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "Microsoft-Windows-TerminalServices-RemoteConnectionManager" => {
                EventProvider::TerminalServicesRemoteConnectionManager
            }
            "Microsoft-Windows-TerminalServices-LocalSessionManager" => {
                EventProvider::TerminalServicesLocalSessionManager
            }
            "Microsoft-Windows-RemoteDesktopServices-RdpCoreTS" => {
                EventProvider::RemoteDesktopServicesRdpCoreTS
            }
            "Microsoft-Windows-Security-Auditing" => EventProvider::SecurityAuditing,
            "Desktop Window Manager" => EventProvider::DesktopWindowManager,
            _ => {
                //panic!("unknown provider name: {value}");
                log::warn!("unknown provider name: {value}");
                Self::UnsupportedProvider
            }
        })
    }
}

impl FromMeta for EventProvider {
    fn from_string(value: &str) -> darling::Result<Self> {
        match Self::try_from(value) {
            Ok(me) => Ok(me),
            Err(_) => Err(darling::Error::unknown_value(value)),
        }
    }
}

impl quote::ToTokens for EventProvider {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let provider_token = match self {
            EventProvider::TerminalServicesRemoteConnectionManager => {
                quote!(EventProvider::TerminalServicesRemoteConnectionManager)
            }
            EventProvider::TerminalServicesLocalSessionManager => {
                quote!(EventProvider::TerminalServicesLocalSessionManager)
            }
            EventProvider::RemoteDesktopServicesRdpCoreTS => {
                quote!(EventProvider::RemoteDesktopServicesRdpCoreTS)
            }
            EventProvider::SecurityAuditing => quote!(EventProvider::SecurityAuditing),
            EventProvider::DesktopWindowManager => quote!(EventProvider::DesktopWindowManager),
            EventProvider::UnsupportedProvider => quote!(EventProvider::UnsupportedProvider),
        };
        tokens.extend(provider_token)
    }
}
