use binread::BinRead;

/// Possible data types of the data belonging to a [`KeyValue`].
/// https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types
#[derive(BinRead, Clone, Copy)]
#[br(repr=u32)]
pub enum KeyValueDataType {
    /// Data with no particular type
    RegNone = 0x0000_0000,

    /// A null-terminated string. This will be either a Unicode or an ANSI string, depending on whether you use the Unicode or ANSI functions.
    RegSZ = 0x0000_0001,

    /// A null-terminated Unicode string, containing unexpanded references to environment variables, such as "%PATH%"
    RegExpandSZ = 0x0000_0002,

    /// Binary data in any form
    RegBinary = 0x0000_0003,

    /// A 4-byte numerical value
    RegDWord = 0x0000_0004,

    /// A 4-byte numerical value whose least significant byte is at the highest address
    RegDWordBigEndian = 0x0000_0005,

    /// A Unicode string naming a symbolic link. This type is irrelevant to device and intermediate drivers
    RegLink = 0x0000_0006,

    /// An array of null-terminated strings, terminated by another zero
    RegMultiSZ = 0x0000_0007,

    /// A device driver's list of hardware resources, used by the driver or one of the physical devices it controls, in the \ResourceMap tree
    RegResourceList = 0x0000_0008,

    /// A list of hardware resources that a physical device is using, detected and written into the \HardwareDescription tree by the system
    RegFullResourceDescriptor = 0x0000_0009,

    /// A device driver's list of possible hardware resources it or one of the physical devices it controls can use, from which the system writes a subset into the \ResourceMap tree
    RegResourceRequirementsList = 0x0000_000a,

    /// A 64-bit number.
    RegQWord = 0x0000_000b,

    /// FILETIME data
    RegFileTime = 0x0000_0010,
}