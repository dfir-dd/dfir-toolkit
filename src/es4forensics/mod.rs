
//! This crates provides structs and functions to insert timeline data into
//! an elasticsearch index.
//! 
//! # Creating Indices
//! ```
//! use dfir_toolkit::es4forensics::IndexBuilder;
//! use dfir_toolkit::es4forensics::WithHost;
//! use elasticsearch::auth::Credentials;
//! 
//!# #[tokio::main]
//!# async fn main() {  
//! let username = "elastic";
//! let password = "elastic";
//! let credentials = Credentials::Basic(username.to_string(), password.to_string());
//! let mut index = IndexBuilder::with_name("elastic4forensics_test".to_string())
//!     .with_host("127.0.0.1")
//!     .with_port(9200)
//!     .without_certificate_validation()
//!     .with_credentials(credentials)
//!     .create_index().await;
//!# }
//! ```
//! After doing this, you can easily add documents to the index using [`Index::add_timeline_object`]
//! 
//! # Adding documents to elasticsearch
//! 
//! For example, consider we have a line from a bodyfile. We need to convert this
//! into a [`ecs::objects::PosixFile`]-Object, which can then be added to an Index:
//! 
//! ```
//! use dfir_toolkit::es4forensics::objects::PosixFile;
//!# use dfir_toolkit::es4forensics::Index;
//! 
//!# fn foo(mut index: Index) {
//! let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
//! let posix_file: PosixFile = str_line.try_into().unwrap();
//! 
//! index.add_timeline_object(posix_file);
//!# }
//! ```
//! 
//! # Exporting documents in JSON format
//! 
//! Sometimes you might want to simply export your documents, instead of directly importing them into
//! elasticsearch.
//! 
//! Keep in mind that one bodyfile line might contain multiple different timestamps (up to four),
//! which yields up to four elasticsearch documents. Therefore, [`ecs::objects::ElasticObject::documents()`] returns an
//! iterator over [`serde_json::Value`]
//! 
//! ```
//! use dfir_toolkit::es4forensics::objects::PosixFile;
//! use dfir_toolkit::es4forensics::Timestamp;
//! use dfir_toolkit::es4forensics::TimelineObject;
//! use serde_json::Value;
//!# use dfir_toolkit::es4forensics::Index;
//! 
//!# fn foo(mut index: Index) {
//! let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
//! let posix_file: PosixFile = str_line.try_into().unwrap();
//! 
//! for json_value in posix_file.into_values() {
//!     println!("{json_value}");
//! }
//!# }
//! ```

#[cfg(feature="elasticsearch")]
mod index;

#[cfg(feature="elasticsearch")]
mod index_builder;

mod timestamp;
mod utils;
mod ecs;

mod protocol;
mod stream_source;

#[cfg(feature="elasticsearch")]
pub use index::*;

#[cfg(feature="elasticsearch")]
pub use index_builder::*;
pub use timestamp::*;
pub use ecs::*;
pub use protocol::*;
pub use stream_source::*;