use binread::BinRead;

use super::DirtyPagesReference;

/// <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#new-format>
#[derive(BinRead,Debug,Clone,Default)]
#[br(import(size: usize))]
pub struct DirtyPage {
    
    #[br(count(size))]
    data: Vec<u8>
}

impl DirtyPage {
    pub fn new(_reference: &DirtyPagesReference, data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl AsRef<[u8]> for DirtyPage {
    fn as_ref(&self) -> &[u8] {
        &self.data[..]
    }
}