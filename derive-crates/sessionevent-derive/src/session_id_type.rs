use darling::FromMeta;
use quote::{ToTokens, quote};

pub enum SessionIdType {
    ActivityId,
    SessionName,
    LogonId,
    None,
}

impl FromMeta for SessionIdType {
    fn from_string(value: &str) -> darling::Result<Self> {
        if value == "ActivityId" {
            Ok(SessionIdType::ActivityId)
        } else if value == "SessionName" {
            Ok(SessionIdType::SessionName)
        } else if value == "LogonId" {
            Ok(SessionIdType::LogonId)
        } else if value == "None" {
            Ok(SessionIdType::None)
        } else {
            Err(darling::Error::unknown_value(value))
        }
    }
}

impl ToTokens for SessionIdType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let generator = match self {
            SessionIdType::ActivityId => quote!(SessionNameInActivityId),
            SessionIdType::SessionName => quote!(SessionNameInLogonId),
            SessionIdType::LogonId => quote!(SessionNameInLogonId),
            SessionIdType::None => quote!(NoSessionId),
        };
        tokens.extend(generator)
    }
}
