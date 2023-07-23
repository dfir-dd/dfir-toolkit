use std::path::PathBuf;

use duplicate::duplicate_item;
use serde::Serialize;

use crate::es4forensics::timestamp::Timestamp;

use super::ecs_object::EcsObject;

#[derive(Serialize)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Serialize, Default)]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    mtime: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessed: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ctime: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<String>,
    gid: u64,
    uid: u64,
    inode: String,
    mode: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    size: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_path: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    file_type: Option<FileType>,
}

impl From<String> for File {
    fn from(filename: String) -> Self {
        let buf = PathBuf::from(&filename);
        Self {
            name: buf.file_name().map(|s| s.to_string_lossy().to_string()),
            extension: buf.extension().map(|s| s.to_string_lossy().to_string()),
            directory: buf.parent().map(|s| s.to_string_lossy().to_string()),
            path: Some(filename),
            ..Default::default()
        }
    }
}

impl File {
    #[duplicate_item(
        method            attribute    ret_type;
      [ with_mtime ]       [ mtime ]       [ Timestamp ];
      [ with_accessed ]    [ accessed ]    [ Timestamp ];
      [ with_ctime ]       [ ctime ]       [ Timestamp ];
      [ with_created ]     [ created ]     [ Timestamp ];
      [ with_target_path ] [ target_path ] [ String ];
      [ with_type ]        [ file_type ]   [ FileType ];
   )]
    pub fn method(mut self, ts: Option<ret_type>) -> Self {
        self.attribute = ts;
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_gid ]   [ gid ]   [ u64 ];
      [ with_uid ]   [ uid ]   [ u64 ];
      [ with_inode ] [ inode ] [ String ];
      [ with_mode ]  [ mode ]  [ String ];
      [ with_size ]  [ size ]  [ u64 ];
   )]
    pub fn method(mut self, ts: ret_type) -> Self {
        self.attribute = ts;
        self
    }
}

impl EcsObject for File {
    fn object_key(&self) -> &'static str {
        "file"
    }
}
