use std::fmt::Display;

use anyhow::bail;
use darling::FromMeta;
use evtx::SerializedEvtxRecord;
use quote::quote;
use serde::Serialize;
use serde_json::Value;

use super::EvtxFieldView;

#[derive(PartialEq, Eq, Clone, Debug, Serialize)]
pub struct ProcessId(pub u64);

impl TryFrom<&SerializedEvtxRecord<Value>> for ProcessId {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let process_id = &record.data["Event"]["System"]["Execution"]["#attributes"]["ProcessID"];

        let process_id = match process_id.get("#text") {
            Some(eid) => eid,
            None => process_id,
        };

        if let Some(process_id) = process_id.as_u64() {
            let id: u64 = process_id.try_into()?;
            Ok(Self(id))
        } else {
            bail!("event id cannot be converted to u64: {process_id}")
        }
    }
}

impl ProcessId {
    pub fn value(&self) -> u64 {
        self.0
    }
}

pub const PROCESS_ID_MAX_LENGTH: usize = 10;
impl EvtxFieldView for ProcessId {
    fn maximum_display_length(&self) -> usize {
        PROCESS_ID_MAX_LENGTH
    }

    fn value_with_padding(&self) -> String {
        format!("{:10}", self.0)
    }
}

impl From<ProcessId> for u64 {
    fn from(me: ProcessId) -> Self {
        me.0
    }
}

impl From<u64> for ProcessId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromMeta for ProcessId {
    fn from_value(value: &darling::export::syn::Lit) -> darling::Result<Self> {
        match value {
            darling::export::syn::Lit::Int(lit) => Ok(Self::from(lit.base10_parse::<u64>()?)),
            _ => Err(darling::Error::unknown_value("invalid process id")),
        }
    }
}

impl quote::ToTokens for ProcessId {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let me = self.0;
        tokens.extend(quote!(
            {
                use eventdata::ProcessId;
                ProcessId(#me)
            }
        ))
    }
}