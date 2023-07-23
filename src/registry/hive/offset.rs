use binread::BinRead;

/// represents an offset (usually a 32bit value) used in registry hive files
#[derive(BinRead, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Offset(pub u32);
