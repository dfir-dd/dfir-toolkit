use binread::derive_binread;

use super::Offset;

#[derive_binread]
#[br(magic=b"hbin")]
#[allow(dead_code)]
pub (crate) struct HiveBin {
    offset: Offset,
    size: u32,
    reserved: u64,
    timestamp: u64,
    spare: u32
}

impl HiveBin {
    pub fn size(&self) -> u32 {
        self.size
    }
}