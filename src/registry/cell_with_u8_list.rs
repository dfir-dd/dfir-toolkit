use super::CellHeader;
use binread::derive_binread;

#[derive_binread]
#[br(import(count:Option<usize>))]
pub(crate) struct CellWithU8List {
    #[br(temp)]
    header: CellHeader,

    #[br(count=count.or_else(|| Some(header.contents_size())).unwrap())]
    pub data: Vec<u8>,
}

impl From<CellWithU8List> for Vec<u8> {
    fn from(cell: CellWithU8List) -> Self {
        cell.data
    }
}
