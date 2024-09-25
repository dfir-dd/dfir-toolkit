use std::io::Write;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use dfir_toolkit::common::bodyfile::BehavesLikeI64;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use flow_record::artifacts::posix::FileMode;
use flow_record::derive::FlowRecord;
use flow_record::{artifacts::posix::FileType, prelude::*};
use types::PathType;
use types::{Filesize, Path};

use crate::bodyfile::{ListEntry, Mactime2Writer};

pub(crate) struct RecordOutput<W>
where
    W: Write + Send,
{
    _dst_zone: Tz,
    writer: Serializer<W>,
}

impl<W> RecordOutput<W>
where
    W: Write + Send,
{
    pub fn new(writer: W, _dst_zone: Tz) -> Self {
        Self {
            _dst_zone,
            writer: Serializer::new(writer),
        }
    }
    #[allow(dead_code)]
    pub fn with_writer(mut self, writer: W) -> Self
    where
        W: Write + Send + 'static,
    {
        self.writer = Serializer::new(writer);
        self
    }
}

impl<W> Mactime2Writer<W> for RecordOutput<W>
where
    W: Write + Send,
{
    fn write_line(&mut self, _timestamp: &i64, entry: &ListEntry) -> std::io::Result<()> {
        let record = FileRecord::try_from(entry.line.as_ref()).expect("invalid bodyfile data");
        self.writer.serialize(record).unwrap();
        Ok(())
    }

    fn into_writer(self) -> W {
        self.writer.into_inner()
    }
}

#[derive(FlowRecord)]
#[flow_record(version = 1, source = "Posix", classification = "file")]
pub struct FileRecord {
    file_name: Path,
    user_id: u64,
    group_id: u64,
    file_type: FileType,
    mode: FileMode,
    size: Filesize,

    modified: Option<DateTime<Utc>>,
    accessed: Option<DateTime<Utc>>,
    changed: Option<DateTime<Utc>>,
    birth: Option<DateTime<Utc>>,
}

struct UnixTimestamp(i64);

impl From<i64> for UnixTimestamp {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UnixTimestamp> for Option<DateTime<Utc>> {
    fn from(value: UnixTimestamp) -> Self {
        if value.0 != -1 {
            DateTime::from_timestamp(value.0, 0)
        } else {
            None
        }
    }
}

impl TryFrom<&Bodyfile3Line> for FileRecord {
    type Error = flow_record::prelude::Error;
    fn try_from(line: &Bodyfile3Line) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name: Path::new(line.get_name().to_string().into(), PathType::Posix),
            user_id: *line.get_uid(),
            group_id: *line.get_gid(),
            mode: FileMode::try_from(&line.get_mode_as_string()[..])?,
            file_type: FileType::try_from(&line.get_mode_as_string()[..])?,
            size: Filesize::from(*line.get_size()),
            modified: line
                .get_mtime()
                .as_ref()
                .and_then(|t| Option::<DateTime<Utc>>::from(UnixTimestamp::from(*t))),
            accessed: line
                .get_atime()
                .as_ref()
                .and_then(|t| Option::<DateTime<Utc>>::from(UnixTimestamp::from(*t))),
            changed: line
                .get_ctime()
                .as_ref()
                .and_then(|t| Option::<DateTime<Utc>>::from(UnixTimestamp::from(*t))),
            birth: line
                .get_crtime()
                .as_ref()
                .and_then(|t| Option::<DateTime<Utc>>::from(UnixTimestamp::from(*t))),
        })
    }
}
