use binread::{derive_binread, BinRead};

use super::{Offset, Cell};

/// On-Disk Structure of a Subkeys List header.
/// This is common for all subkey types (Fast Leaf, Hash Leaf, Index Leaf, Index Root).
#[derive_binread]
#[br(little)]
#[derive(Debug)]
pub(crate) enum SubKeysList {

    #[br(magic = b"li")] IndexLeaf{
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<IndexLeafItem>
    },

    #[br(magic = b"lf")] FastLeaf{
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<FastLeafItem>
    },

    #[br(magic = b"lh")] HashLeaf{
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<HashLeafItem>
    },

    #[br(magic = b"ri")] IndexRoot{
        #[br(temp)]
        count: u16,

        #[br(count=count)]
        items: Vec<IndexRootListElement>
    },
}

impl SubKeysList {
    #[allow(unused)]
    pub fn offsets<'a>(&'a self) -> Box<dyn Iterator<Item=Offset> + 'a> {
        match self {
            SubKeysList::IndexLeaf { items, ..} => Box::new(items.iter().map(|i| i.key_node_offset)),
            SubKeysList::FastLeaf { items , ..} => Box::new(items.iter().map(|i| i.key_node_offset)),
            SubKeysList::HashLeaf { items , ..} => Box::new(items.iter().map(|i| i.key_node_offset)),
            SubKeysList::IndexRoot { items , ..} => Box::new(items.iter().map(|i| i.subkeys_list_offset)),
        }
    }

    pub fn into_offsets(self) -> Box<dyn Iterator<Item=Offset>> {
        match self {
            SubKeysList::IndexLeaf { items, ..} => Box::new(items.into_iter().map(|i| i.key_node_offset)),
            SubKeysList::FastLeaf { items , ..} => Box::new(items.into_iter().map(|i| i.key_node_offset)),
            SubKeysList::HashLeaf { items , ..} => Box::new(items.into_iter().map(|i| i.key_node_offset)),
            SubKeysList::IndexRoot { items , ..} => Box::new(items.into_iter().map(|i| i.subkeys_list_offset)),
        }
    }

    pub fn is_index_root(&self) -> bool {
        matches!(self, SubKeysList::IndexRoot { items: _ , ..})
    }
}

impl From<Cell<SubKeysList, ()>> for SubKeysList {
    fn from(cell: Cell<SubKeysList, ()>) -> Self {
        cell.into_data()
    }
}

#[derive(BinRead, Debug)]
pub struct HashLeafItem {
    key_node_offset: Offset,

    #[allow(unused)]
    name_hash: [u8; 4],
}

#[derive(BinRead, Debug)]
pub struct FastLeafItem {
    key_node_offset: Offset,

    #[allow(unused)]
    name_hint: [u8; 4],
}

#[derive(BinRead, Debug)]
pub struct IndexRootListElement {
    subkeys_list_offset: Offset
}

#[derive(BinRead, Debug)]
pub struct IndexLeafItem {
    key_node_offset: Offset,
}