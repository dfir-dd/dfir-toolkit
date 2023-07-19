use std::fmt::Display;

use anyhow::bail;
use darling::FromMeta;
use evtx::SerializedEvtxRecord;
use quote::quote;
use serde::Serialize;
use serde_json::Value;

use super::EvtxFieldView;

#[derive(PartialEq, Eq, Clone, Debug, Serialize)]
pub struct EventId(pub u16);

impl TryFrom<&SerializedEvtxRecord<Value>> for EventId {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let event_id = &record.data["Event"]["System"]["EventID"];

        let event_id = match event_id.get("#text") {
            Some(eid) => eid,
            None => event_id,
        };

        if let Some(event_id) = event_id.as_u64() {
            let id: u16 = event_id.try_into()?;
            Ok(Self(id))
        } else {
            bail!("event id cannot be converted to u16: {event_id}")
        }
    }
}

impl EventId {
    pub fn value(&self) -> u16 {
        self.0
    }
}

const EVENT_ID_MAX_LENGTH: usize = 5;
impl EvtxFieldView for EventId {
    fn maximum_display_length(&self) -> usize {
        EVENT_ID_MAX_LENGTH
    }

    fn value_with_padding(&self) -> String {
        format!("{:5}", self.0)
    }
}

impl From<EventId> for u16 {
    fn from(me: EventId) -> Self {
        me.0
    }
}

impl From<u16> for EventId {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromMeta for EventId {
    fn from_value(value: &darling::export::syn::Lit) -> darling::Result<Self> {
        match value {
            darling::export::syn::Lit::Int(lit) => Ok(Self::from(lit.base10_parse::<u16>()?)),
            _ => Err(darling::Error::unknown_value("invalid event id")),
        }
    }
}

impl quote::ToTokens for EventId {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let me = self.0;
        tokens.extend(quote!(
            {
                use eventdata::EventId;
                EventId(#me)
            }
        ))
    }
}