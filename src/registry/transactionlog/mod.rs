use std::{io::{Read, Seek, ErrorKind}, fs::File};

use binread::{BinRead, ReadOptions, BinResult, BinReaderExt};
use derive_getters::Getters;

use crate::registry::hive::{HiveBaseBlock, FileType};

pub use self::transactionlogsentry::TransactionLogsEntry;

mod transactionlogsentry;
mod dirty_pages;
mod application_result;

pub use dirty_pages::*;
pub use application_result::*;

// <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#new-format>
#[derive(BinRead, Debug, Clone, Default, Getters)]
pub struct TransactionLog {

    /// A modified partial backup copy of a base block is stored in the first
    /// sector of a transaction log file in the same way as in the old format
    /// and for the same purpose. However, the File type field is set to 6.
    #[br(try, args(FileType::TransactionLogVariant3,))]
    base_block: Option<HiveBaseBlock>,

    #[br(parse_with=read_log_entries, assert(!log_entries.is_empty()))]
    log_entries: Vec<TransactionLogsEntry>
}


fn read_log_entries<R: Read + Seek>(
    reader: &mut R,
    _ro: &ReadOptions,
    _params: (),
) -> BinResult<Vec<TransactionLogsEntry>> {
    let mut log_entries = Vec::new();

    // read until an error occurs
    loop {
        match reader.read_le::<TransactionLogsEntry>() {
            Ok(entry) => {
                log::info!("found transaction log entry with seq# {}", entry.sequence_number());
                log_entries.push(entry)
            }
            Err(why) => {
                if let binread::Error::Io(kind) = &why {
                    if kind.kind() == ErrorKind::UnexpectedEof {
                        log::info!("stop reading after {} entries", log_entries.len());
                        return Ok(log_entries);
                    }
                }
                log::warn!("error while reading transaction log entry: {why}");
                return Ok(log_entries);
            }
        }
    }
}

impl From<TransactionLog> for Vec<TransactionLogsEntry> {
    fn from(log: TransactionLog) -> Self {
        log.log_entries
    }
}

impl TryFrom<File> for TransactionLog {
    type Error = binread::Error;

    fn try_from(mut file: File) -> Result<Self, Self::Error> {
        file.read_le::<TransactionLog>()
    }
}

impl IntoIterator for TransactionLog {
    type Item = TransactionLogsEntry;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.log_entries.into_iter()
    }
}