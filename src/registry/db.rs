use binread::{derive_binread, ReadOptions, BinResult, FilePtr32};
use std::io::{Read, Seek};

use super::{CellHeader, cell_with_u8_list::CellWithU8List};

pub const BIGDATA_MAX_SEGMENT_SIZE: u16 = 16344;

#[derive_binread]
#[br(magic = b"db")]
pub struct BigData {
    #[br(temp)]
    segments_count: u16,

    #[br(temp, deref_now, args(segments_count,))]
    segments: FilePtr32<SegmentList>,

    #[br(parse_with=obtain_data_bytes, args(&segments,))]
    pub bytes: Vec<u8>
}

#[derive_binread]
#[br(import(count:u16))]
struct SegmentList {
    #[br(temp)]
    header: CellHeader,

    #[br(count=count, args(None,))]
    pub segments: Vec<FilePtr32<CellWithU8List>>
}

fn obtain_data_bytes<R: Read + Seek>(
    _reader: &mut R,
    _ro: &ReadOptions,
    args: (&FilePtr32<SegmentList>,),
) -> BinResult<Vec<u8>> {
    let segment_list = args.0.value.as_ref().unwrap();

    // allocate the maximum expected size of data
    let mut res = Vec::with_capacity(segment_list.segments.len() * BIGDATA_MAX_SEGMENT_SIZE as usize);

    for item_ptr in &segment_list.segments {
        let item = item_ptr.value.as_ref().unwrap();
        let data: &Vec<u8> = &item.data;
        res.extend(data);
    }
    
    Ok(res)
}