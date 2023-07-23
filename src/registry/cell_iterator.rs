use std::io::{ErrorKind, Seek, SeekFrom};

use binread::{derive_binread, BinRead, BinReaderExt, BinResult};
use thiserror::Error;

use crate::registry::{CellHeader, Offset, KeyNode, KeyValue};

use super::hive::CleanHive;
use super::hivebin::HiveBin;
use super::{subkeys_list::*, Hive};

pub enum CellFilter {
    DeletedOnly,
    AllocatedOnly,
    DeletedAndAllocated,
}

impl Default for CellFilter {
    fn default() -> Self {
        Self::DeletedAndAllocated
    }
}

pub struct CellIterator<B, C>
where
    B: BinReaderExt,
    C: Fn(u64),
{
    hive: Hive<B, CleanHive>,
    hivebin: Option<HiveBin>,
    read_from_hivebin: usize,
    callback: C,
    filter: CellFilter,
}

impl<B, C> CellIterator<B, C>
where
    B: BinReaderExt,
    C: Fn(u64),
{
    pub fn new(mut hive: Hive<B, CleanHive>, callback: C) -> Self {
        hive.seek(SeekFrom::Start(0)).unwrap();
        Self {
            hive,
            hivebin: None,
            read_from_hivebin: 0,
            callback,
            filter: CellFilter::default(),
        }
    }

    pub fn with_filter(mut self, filter: CellFilter) -> Self {
        self.filter = filter;
        self
    }

    fn read_hivebin_header(&mut self) -> BinResult<()> {
        match self.hive.read_le::<HiveBin>() {
            Err(why) => {
                if let binread::Error::Io(kind) = &why {
                    if kind.kind() == ErrorKind::UnexpectedEof {
                        log::warn!("unexpected EOF while trying to read hivebin header");
                        return Err(why);
                    }
                }
                log::warn!("parser error: {}", why);
                Err(why)
            }
            Ok(hivebin) => {
                self.hivebin = Some(hivebin);
                self.read_from_hivebin = 0;
                Ok(())
            }
        }
    }
}

impl<B, C> Iterator for CellIterator<B, C>
where
    B: BinReaderExt,
    C: Fn(u64),
{
    type Item = CellSelector;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.hivebin.is_none() && self.read_hivebin_header().is_err() {
                return None;
            }

            let start_position = self.hive.stream_position().unwrap();

            // there might be the start of a new hive bin at this position
            if start_position & (!0xfff) == start_position {
                log::debug!(
                    "trying to read hivebin header at {:08x}",
                    start_position + 0x1000
                );

                match self.hive.read_le::<HiveBin>() {
                    Ok(hivebin) => {
                        log::debug!("found a new hivebin here");
                        self.hivebin = Some(hivebin);
                        self.read_from_hivebin = 0;
                    }
                    Err(why) => {
                        log::debug!(
                            "this does not seem to be a hivebin header (cause was: {})",
                            why
                        );
                    }
                }

                (self.callback)(self.hive.stream_position().unwrap());
            }

            let start_position = self.hive.stream_position().unwrap();
            log::trace!("reading a cell at {:08x}", start_position + 4096);

            let result: BinResult<CellHeader> = self.hive.read_le();

            match result {
                Err(why) => {
                    if let binread::Error::Io(kind) = &why {
                        if kind.kind() == ErrorKind::UnexpectedEof {
                            return None;
                        }
                    }
                    log::warn!("parser error: {}", why);
                    (self.callback)(self.hive.stream_position().unwrap());
                    return None;
                }

                Ok(header) => {
                    let handle_this_cell = match self.filter {
                        CellFilter::DeletedOnly => header.is_deleted(),
                        CellFilter::AllocatedOnly => !header.is_deleted(),
                        CellFilter::DeletedAndAllocated => true,
                    };

                    if !handle_this_cell {
                        self.hive
                            .seek(SeekFrom::Start(header.size() as u64 + start_position))
                            .unwrap();
                        continue;
                    }

                    let result: BinResult<CellLookAhead> = self.hive.read_le();
                    match result {
                        Err(why) => {
                            if let binread::Error::Io(kind) = &why {
                                if kind.kind() == ErrorKind::UnexpectedEof {
                                    return None;
                                }
                            }
                            log::warn!("parser error: {}", why);
                            (self.callback)(self.hive.stream_position().unwrap());
                            return None;
                        }

                        Ok(content) => {
                            if self.read_from_hivebin + header.size()
                                >= self.hivebin.as_ref().unwrap().size().try_into().unwrap()
                            {
                                // the hivebin has been completely read, the next to be read should be
                                // the next hivebin header
                                log::trace!("the current hivebin has been completely read");
                                self.hivebin = None;
                            }

                            log::trace!(
                                "skipping {} bytes to {:08x}",
                                header.size(),
                                start_position as usize + header.size()
                            );

                            self.hive
                                .seek(SeekFrom::Start(header.size() as u64 + start_position))
                                .unwrap();
                            (self.callback)(self.hive.stream_position().unwrap());
                            return Some(CellSelector {
                                offset: Offset(start_position.try_into().unwrap()),
                                header,
                                content,
                            });
                        }
                    }
                }
            }
        }
    }
}

#[derive(BinRead)]
pub struct CellSelector {
    offset: Offset,
    header: CellHeader,
    content: CellLookAhead,
}

impl CellSelector {
    pub fn offset(&self) -> &Offset {
        &self.offset
    }
    pub fn header(&self) -> &CellHeader {
        &self.header
    }
    pub fn content(&self) -> &CellLookAhead {
        &self.content
    }
}

#[derive_binread]
pub enum CellLookAhead {
    #[br(magic = b"nk")]
    NK(KeyNode),
    #[br(magic = b"vk")]
    VK(KeyValue),
    #[br(magic = b"sk")]
    SK,
    #[br(magic = b"db")]
    DB,

    #[br(magic = b"li")]
    LI {
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<IndexLeafItem>,
    },
    #[br(magic = b"lf")]
    LF {
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<FastLeafItem>,
    },

    #[br(magic = b"lh")]
    LH {
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<HashLeafItem>,
    },
    #[br(magic = b"ri")]
    RI {
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<IndexRootListElement>,
    },
    UNKNOWN,
}

#[derive(Error, Debug)]
pub enum CellLookAheadConversionError {
    #[error(
        "tried to extract some type from this cell, which is not actually stored in this cell."
    )]
    DifferentCellTypeExpected,
}

impl CellLookAhead {
    pub fn is_nk(&self) -> bool {
        matches!(self, Self::NK(_))
    }
}

impl TryInto<KeyNode> for CellSelector {
    type Error = CellLookAheadConversionError;

    fn try_into(self) -> Result<KeyNode, Self::Error> {
        match self.content {
            CellLookAhead::NK(nk) => Ok(nk),
            _ => Err(CellLookAheadConversionError::DifferentCellTypeExpected),
        }
    }
}
