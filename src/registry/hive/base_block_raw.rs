use binread::BinRead;


#[allow(dead_code)]
#[derive(BinRead)]
pub (crate) struct HiveBaseBlockRaw {
    #[br(count = 127)]
    raw_data: Vec<u32>,

    #[br(assert(checksum_of(&raw_data[..]) == checksum, "expected checksum of 0x{:08x}, but found 0x{checksum:08x} instead", checksum_of(&raw_data[..])))]
    checksum: u32,

    #[br(count = 0x37E)]
    padding_2: Vec<u32>,
    boot_type: u32,
    boot_recover: u32,
}

/// <https://systemroot.gitee.io/pages/apiexplorer/d9/d1/hivesum_8c.html#a0>
fn checksum_of(bytes: &[u32]) -> u32 {
    match bytes.iter().fold(0, |acc, x| acc ^ x) {
        0xffff_ffff => 0xffff_fffe,
        0           => 1,
        sum    => sum
    }
}