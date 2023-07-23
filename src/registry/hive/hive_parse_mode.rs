use crate::registry::Offset;


pub enum HiveParseMode {
    /// to be used only when converting this hive to an iterator
    Raw,

    /// to be used if you don't expect a usable base block
    Normal(Offset),

    /// for normal parsing of registry files
    NormalWithBaseBlock
}