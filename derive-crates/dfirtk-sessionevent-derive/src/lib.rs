use darling::FromDeriveInput;
use dfirtk_eventdata::{EventProvider, EventId};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod session_id_type;
use session_id_type::*;

#[derive(FromDeriveInput)]
#[darling(attributes(event_data))]
struct EventStructOptions {
    provider: EventProvider,
    event_id: EventId,
    description: String,
    session_id: SessionIdType,
    username_path: Option<String>,
    domain_path: Option<String>,
    client_hostname_path: Option<String>,
    client_address_path: Option<String>,
    server_hostname_path: Option<String>,
    server_address_path: Option<String>,
}

fn create_getter(path: &Option<String>, function_name: TokenStream) -> TokenStream {
    match path {
        None => quote!(
            fn #function_name (&self, _: &SerializedEvtxRecord<Value>) -> Option<String> {
                None
            }),
        Some(path) => {
            let parts: Vec<_> = path.split('/').map(|part| quote!([#part])).collect();
            quote!(
                fn #function_name (&self, record: &SerializedEvtxRecord<Value>) -> Option<String> {
                    record.data #(#parts)* .as_str().map(|s|s.to_owned())
                })
        }
    }
}

#[proc_macro_derive(SessionEvent, attributes(event_data))]
pub fn derive_session_event(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = EventStructOptions::from_derive_input(&input).expect("Wrong options");
    let name = input.ident;

    let provider = opts.provider;
    let event_id = opts.event_id;
    let description = opts.description;
    let session_id_type = opts.session_id;

    let username_getter = create_getter(&opts.username_path, quote!(username));
    let domain_getter = create_getter(&opts.domain_path, quote!(domain));
    let client_hostname_getter = create_getter(&opts.client_hostname_path, quote!(client_hostname));
    let client_address_getter = create_getter(&opts.client_address_path, quote!(client_address));
    let server_hostname_getter = create_getter(&opts.server_hostname_path, quote!(server_hostname));
    let server_address_getter = create_getter(&opts.server_address_path, quote!(server_address));

    let expanded = quote! {
        impl SessionEventInfo for #name {
            fn event_id(&self) -> EventId {
                #event_id
            }
            fn description(&self) -> &'static str {
                #description
            }
            fn provider(&self) -> EventProvider {
                #provider
            }
            fn generate_id(&self, record: &SerializedEvtxRecord<Value>) -> dfirtk_eventdata::SessionId {
                #session_id_type::session_id_of(record)
            }
            #username_getter
            #domain_getter
            #client_hostname_getter
            #client_address_getter
            #server_hostname_getter
            #server_address_getter
        }
    };

    proc_macro::TokenStream::from(expanded)
}
