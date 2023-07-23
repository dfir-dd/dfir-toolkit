use std::{
    hash::Hasher,
    io::{Read, Seek},
};

use binread::{BinRead, BinResult, ReadOptions};
use derive_getters::Getters;
use marvin32::Marvin32;

use super::dirty_pages::{DirtyPage, DirtyPagesReference};

pub const BLOCK_SIZE: u32 = 512;
#[allow(dead_code)]
pub const HVLE_START_OFFSET: u64 = 512;
pub const HIVE_BIN_SIZE_ALIGNMENT: u32 = 4096;
#[allow(dead_code)]
pub const BASE_BLOCK_LENGTH_PRIMARY: u32 = 4096;
#[allow(dead_code)]
pub const HBIN_START_OFFSET: u64 = 600;

#[derive(BinRead, Debug, Clone, Default, Getters)]
pub struct Marvin32Hash {
    p0: u32,
    p1: u32,
}

impl Marvin32Hash {
    pub fn collapse(&self) -> u32 {
        self.p0 ^ self.p1
    }
}

impl From<&Marvin32Hash> for u32 {
    fn from(value: &Marvin32Hash) -> Self {
        value.collapse()
    }
}

impl From<Marvin32Hash> for u32 {
    fn from(value: Marvin32Hash) -> Self {
        value.collapse()
    }
}

/// <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#new-format>
#[derive(BinRead, Debug, Clone, Default, Getters)]
#[br(magic = b"HvLE", assert(hash1.collapse() == calc_hash1(&dirty_pages_references, &dirty_pages, &slack), "expected 0x{:08x}", hash1.collapse()))]
pub struct TransactionLogsEntry {
    /// Size of a current log entry in bytes
    #[br(assert(size > BLOCK_SIZE || size % BLOCK_SIZE == 0))]
    size: u32,

    /// Partial copy of the Flags field of the base block at the time of
    /// creation of a current log entry (see below)
    ///
    /// The Flags field of a log entry is set to 0x00000001 when a value of
    /// the Flags field of the base block has the bit mask 0x00000001 set,
    /// otherwise the Flags field of a log entry is set to 0x00000000. During
    /// recovery, the bit mask 0x00000001 is set or unset in the Flags field of
    /// the base block according to a value taken from a log entry being
    /// applied. This means that only the bit mask 0x00000001 is saved to or
    /// restored from a log entry.
    #[br(assert(flags==0))]
    flags: u32,

    /// This number constitutes a possible value of the Primary sequence number
    /// and Secondary sequence number fields of the base block in memory after
    /// a current log entry is applied (these fields are not modified before
    /// the write operation on the recovered hive)
    ///
    /// If a log entry with a sequence number N is not followed by a log entry
    /// with a sequence number N + 1, recovery stops after applying a log entry
    /// with a sequence number N. If the first log entry doesn't contain an
    /// expected sequence number (equal to a primary sequence number of the
    /// base block in a transaction log file, not less than a secondary
    /// sequence number of the valid base block in a primary file), recovery
    /// stops.
    sequence_number: u32,

    /// Copy of the Hive bins data size field of the base block at the time of
    /// creation of a current log entry
    #[br(assert(hbin_data_size > HIVE_BIN_SIZE_ALIGNMENT || hbin_data_size % HIVE_BIN_SIZE_ALIGNMENT == 0))]
    hbin_data_size: u32,

    /// Number of dirty pages attached to a current log entry
    #[br(assert(dirty_pages_count != 0))]
    dirty_pages_count: u32,

    /// Hash-1 is the Marvin32 hash of the data starting from the beginning of
    /// the first page reference of a current log entry with the length of
    /// Size - 40 bytes.
    ///
    /// If a log entry has a wrong value in the field Hash-1, Hash-2, or Hive
    /// bins data size (i.e. it isn't multiple of 4096 bytes), recovery stops,
    /// only previous log entries (preceding a bogus one) are applied.
    hash1: Marvin32Hash,

    /// Hash-2 is the Marvin32 hash of the first 32 bytes of a current log
    /// entry (including the Hash-1 calculated before).
    #[br(assert(hash2.collapse() == calc_hash2(vec![size, flags, sequence_number, hbin_data_size, dirty_pages_count, *hash1.p0(), *hash1.p1()]), "expected 0x{:08x}", hash2.collapse()))]
    hash2: Marvin32Hash,

    /// A dirty page reference describes a single page to be written to a
    /// primary file, and it has the following structure:
    /// | Offset | Field | Length | Description |
    /// |-|-|-|-|
    /// |0|4|Offset|Offset of a page in a primary file (in bytes), relative from the start of the hive bins data|
    /// |4|4|Size|Size of a page in bytes|
    #[br(count = dirty_pages_count,
            assert(dirty_pages_references.len() == <u32 as std::convert::TryInto<usize>>::try_into(dirty_pages_count).unwrap()))]
    dirty_pages_references: Vec<DirtyPagesReference>,

    #[br(parse_with = read_dirty_pages, args(&dirty_pages_references[..]),
            assert(dirty_pages_references.len() == dirty_pages.len()))]
    dirty_pages: Vec<DirtyPage>,

    #[br(calc = ((dirty_pages_references.len() * 8) + dirty_pages.iter().fold(0, |acc, x| acc+x.as_ref().len())).try_into().unwrap())]
    payload_size: u32,

    /// this is required to calculate the hash-1
    #[br(count = size - (40 + payload_size))]
    slack: Vec<u8>,
}

impl From<TransactionLogsEntry> for Vec<DirtyPagesReference> {
    fn from(entry: TransactionLogsEntry) -> Self {
        entry.dirty_pages_references
    }
}

impl From<TransactionLogsEntry> for Vec<DirtyPage> {
    fn from(entry: TransactionLogsEntry) -> Self {
        entry.dirty_pages
    }
}

fn read_dirty_pages<R: Read + Seek>(
    reader: &mut R,
    _ro: &ReadOptions,
    params: (&[DirtyPagesReference],),
) -> BinResult<Vec<DirtyPage>> {
    let mut dirty_pages = Vec::new();
    for dirty_pages_reference in params.0 {
        // allocate memory
        let mut data = vec![0; *dirty_pages_reference.size() as usize];

        // obtain the data based on the length of the page size
        reader.read_exact(data.as_mut_slice())?;

        dirty_pages.push(DirtyPage::new(dirty_pages_reference, data));
    }
    Ok(dirty_pages)
}

fn calc_hash1(
    dirty_pages_references: &Vec<DirtyPagesReference>,
    dirty_pages: &Vec<DirtyPage>,
    slack: &[u8],
) -> u32 {
    let mut hasher = Marvin32::new(0x82EF4D887A4E55C5);
    for reference in dirty_pages_references {
        hasher.write_u32(reference.offset().0);
        hasher.write_u32(*reference.size());
    }
    for page in dirty_pages {
        hasher.write(page.as_ref());
    }
    hasher.write(slack);
    hasher.finish().try_into().unwrap()
}
fn calc_hash2(header_fields: Vec<u32>) -> u32 {
    let mut hasher = Marvin32::new(0x82EF4D887A4E55C5);
    hasher.write(b"HvLE");
    for field in header_fields {
        hasher.write_u32(field);
    }
    hasher.finish().try_into().unwrap()
}
