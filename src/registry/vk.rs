use binread::derive_binread;
use binread::BinResult;
use binread::ReadOptions;
use binread::{BinRead, BinReaderExt};
use bitflags::bitflags;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::registry::db::BigData;
use crate::registry::CellHeader;

use super::cell_with_u8_list::CellWithU8List;
use super::util::parse_reg_multi_sz;
use super::util::parse_reg_sz;
use super::util::parse_string;
use super::util::without_first_bit;
use super::util::BIG_DATA_SEGMENT_SIZE;
use super::util::INV_U32_FIRST_BIT;
use super::util::U32_FIRST_BIT;
use super::{Cell, Offset};
use crate::registry::util::HasFirstBitSet;

#[derive(BinRead, Eq, PartialEq)]
#[br(import(count: usize))]
pub struct KeyValueList {
    #[br(count=count)]
    pub key_value_offsets: Vec<Offset>,
}

pub type KeyValueCell = Cell<KeyValueList, (usize,)>;

impl From<KeyValueCell> for KeyValueList {
    fn from(cell: KeyValueCell) -> Self {
        cell.into_data()
    }
}

bitflags! {
    #[allow(non_upper_case_globals)]
    pub struct KeyValueFlags: u16 {
        /// The name is in (extended) ASCII instead of UTF-16LE.
        const VALUE_COMP_NAME = 0x0001;

        /// Is a tombstone value (the flag is used starting from Insider Preview
        /// builds of Windows 10 "Redstone 1"), a tombstone value also has the
        /// Data type field set to REG_NONE, the Data size field set to 0, and
        /// the Data offset field set to 0xFFFFFFFF
        #[allow(non_upper_case_globals)]
        const IS_TOMBSTONE = 0x0002;
    }
}

#[derive(BinRead)]
#[br(import(data_size: u32))]
pub(crate) enum OffsetOrData {
    /// When the most significant bit is 1, data (4 bytes or less) is stored in
    /// the Data offset field directly (when data contains less than 4 bytes,
    /// it is being stored as is in the beginning of the Data offset field).
    #[br(pre_assert(u32::has_first_bit_set(&data_size) && ( without_first_bit(data_size) == 3 || without_first_bit(data_size) == 4) ))]
    U32Data(u32),

    #[br(pre_assert(u32::has_first_bit_set(&data_size) && without_first_bit(data_size) == 2))]
    U16Data(u16, u16),

    #[br(pre_assert(u32::has_first_bit_set(&data_size) && without_first_bit(data_size) == 1))]
    U8Data(u8, u8, u8, u8),

    #[br(pre_assert(u32::has_first_bit_set(&data_size) && without_first_bit(data_size) == 0))]
    None(u32),

    /// When the most significant bit is 0, data is stored in the Cell data
    /// field of another cell (pointed by the Data offset field) or in the Cell
    /// data fields of multiple cells (referenced in the Big data structure
    /// stored in a cell pointed by the Data offset field).
    #[br(pre_assert(! u32::has_first_bit_set(&data_size)))]
    Offset(Offset),
}

#[derive(BinRead)]
#[br(magic = b"vk")]
pub struct KeyValueWithMagic(KeyValue);

/// Represents a KeyValue as documented in <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#key-value>.
///
#[derive_binread]
#[allow(dead_code)]
pub struct KeyValue {
    name_length: u16,

    #[br(assert(
        [U32_FIRST_BIT, U32_FIRST_BIT | 1, U32_FIRST_BIT | 3, U32_FIRST_BIT | 2, U32_FIRST_BIT | 4].contains(&data_size)
         || ! u32::has_first_bit_set(&data_size), "invalid data size: 0x{:08x}", data_size))]
    data_size: u32,

    #[br(args(data_size))]
    offset_or_data: OffsetOrData,

    /// There are also Types that do not have a value that corresponds to
    /// anything in the list above. These are typically seen in the SAM
    /// Registry hives and often correspond to part of a users SID
    /// <https://binaryforay.blogspot.com/2015/01/registry-hive-basics-part-3-vk-records.html>
    #[br(try)]
    data_type: Option<KeyValueDataType>,

    #[br(temp, if(data_type.is_none()))]
    data_type_spare: u32,

    #[br(parse_with=parse_value_flags)]
    flags: KeyValueFlags,

    #[br(temp)]
    spare: u16,

    #[br(   if(name_length>0, "(Default)".to_string()),
            parse_with=parse_string,
            count=name_length,
            args(flags.contains(KeyValueFlags::VALUE_COMP_NAME)))]
    key_name_string: String,

    #[br(parse_with(parse_registry_value), args(&data_type, &offset_or_data, &data_size))]
    value: RegistryValue,
}

fn parse_registry_value<R: Read + Seek>(
    reader: &mut R,
    _ro: &ReadOptions,
    args: (&Option<KeyValueDataType>, &OffsetOrData, &u32),
) -> BinResult<RegistryValue> {
    let data_type: &Option<KeyValueDataType> = args.0;
    let offset_or_data: &OffsetOrData = args.1;
    let data_size: u32 = args.2 & INV_U32_FIRST_BIT;

    Ok(match offset_or_data {
        OffsetOrData::U32Data(val) => RegistryValue::RegDWord(*val),
        OffsetOrData::U16Data(_, val) => RegistryValue::RegDWord(*val as u32),
        OffsetOrData::U8Data(_, _, _, val) => RegistryValue::RegDWord(*val as u32),
        OffsetOrData::None(_) => RegistryValue::RegNone,
        OffsetOrData::Offset(offset) => {
            match data_type {
                None => RegistryValue::RegUnknown,
                Some(KeyValueDataType::RegNone) => RegistryValue::RegUnknown,
                Some(dt) => {
                    let raw_value = if data_size > BIG_DATA_SEGMENT_SIZE {
                        log::debug!("expecting BIGDATA at 0x{:08x}", offset.0 + 4096);

                        let _offset = reader.seek(SeekFrom::Start(offset.0.into()))?;
                        let _header: CellHeader = reader.read_le()?;
                        let bigdata: BigData = reader.read_le()?;
                        bigdata.bytes
                    } else {
                        // don't treat data as Big Data
                        //eprintln!("reading data of size {} from offset {:08x}", self.data_size(), self.data_offset.val.0 + hive.data_offset());
                        let _offset = reader.seek(SeekFrom::Start(offset.0.into()))?;
                        let data: CellWithU8List =
                            reader.read_le_args((Some(data_size as usize),))?;
                        data.data
                    };

                    match dt {
                        KeyValueDataType::RegNone => RegistryValue::RegNone,
                        KeyValueDataType::RegSZ => {
                            RegistryValue::RegSZ(parse_reg_sz(&raw_value[..])?)
                        }
                        KeyValueDataType::RegExpandSZ => {
                            RegistryValue::RegExpandSZ(parse_reg_sz(&raw_value[..])?)
                        }
                        KeyValueDataType::RegBinary => RegistryValue::RegBinary(raw_value),
                        KeyValueDataType::RegDWord => {
                            RegistryValue::RegDWord(Cursor::new(raw_value).read_le()?)
                        }
                        KeyValueDataType::RegDWordBigEndian => {
                            RegistryValue::RegDWordBigEndian(Cursor::new(raw_value).read_be()?)
                        }
                        KeyValueDataType::RegLink => RegistryValue::RegNone,
                        KeyValueDataType::RegMultiSZ => {
                            RegistryValue::RegMultiSZ(parse_reg_multi_sz(&raw_value[..])?)
                        }
                        KeyValueDataType::RegResourceList => RegistryValue::RegNone,
                        KeyValueDataType::RegFullResourceDescriptor => RegistryValue::RegNone,
                        KeyValueDataType::RegResourceRequirementsList => RegistryValue::RegNone,
                        KeyValueDataType::RegQWord => {
                            RegistryValue::RegQWord(Cursor::new(raw_value).read_le()?)
                        }
                        KeyValueDataType::RegFileTime => RegistryValue::RegFileTime,
                    }
                }
            }
        }
    })
}

/// Possible data types of the data belonging to a [`KeyValue`].
/// https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types
#[derive(BinRead)]
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

impl Display for KeyValueDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let datatype = match self {
            KeyValueDataType::RegNone => "RegNone",
            KeyValueDataType::RegSZ => "RegSZ",
            KeyValueDataType::RegExpandSZ => "RegExpandSZ",
            KeyValueDataType::RegBinary => "RegBinary",
            KeyValueDataType::RegDWord => "RegDWord",
            KeyValueDataType::RegDWordBigEndian => "RegDWordBigEndian",
            KeyValueDataType::RegLink => "RegLink",
            KeyValueDataType::RegMultiSZ => "RegMultiSZ",
            KeyValueDataType::RegResourceList => "RegResourceList",
            KeyValueDataType::RegFullResourceDescriptor => "RegFullResourceDescriptor",
            KeyValueDataType::RegResourceRequirementsList => "RegResourceRequirementsList",
            KeyValueDataType::RegQWord => "RegQWord",
            KeyValueDataType::RegFileTime => "RegFileTime",
        };
        write!(f, "{datatype}")
    }
}

pub enum RegistryValue {
    RegNone,
    RegUnknown,
    RegSZ(String),
    RegExpandSZ(String),
    RegBinary(Vec<u8>),
    RegDWord(u32),
    RegDWordBigEndian(u32),
    RegLink(String),
    RegMultiSZ(Vec<String>),
    RegResourceList(String),
    RegFullResourceDescriptor(String),
    RegResourceRequirementsList(String),
    RegQWord(u64),
    RegFileTime,
}

impl Display for RegistryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryValue::RegUnknown => write!(f, "Unknown"),
            RegistryValue::RegNone => write!(f, "None"),
            RegistryValue::RegSZ(val) => write!(f, "{val:?}"),
            RegistryValue::RegExpandSZ(val) => write!(f, "{val:?}"),
            RegistryValue::RegBinary(val) => {
                write!(f, "{:?}", if val.len() > 16 { &val[..16] } else { val })
            }
            RegistryValue::RegDWord(val) => write!(f, "0x{:08x}", val),
            RegistryValue::RegDWordBigEndian(val) => write!(f, "0x{:08x}", val),
            RegistryValue::RegLink(val) => write!(f, "{val:?}"),
            RegistryValue::RegMultiSZ(val) => write!(f, "{:?}", val),
            RegistryValue::RegResourceList(val) => write!(f, "{val:?}"),
            RegistryValue::RegFullResourceDescriptor(val) => write!(f, "{val:?}"),
            RegistryValue::RegResourceRequirementsList(val) => write!(f, "{val:?}"),
            RegistryValue::RegQWord(val) => write!(f, "0x{:016x}", val),
            RegistryValue::RegFileTime => todo!(),
        }
    }
}

impl KeyValue {
    /// Returns the name of this value
    pub fn name(&self) -> &str {
        &self.key_name_string
    }

    /// Returns [true] if this value is resident, which means that it is stored directly in the offset field.
    pub fn is_resident(&self) -> bool {
        u32::has_first_bit_set(&self.data_size)
    }

    /// Returns the size of the data
    pub fn data_size(&self) -> u32 {
        const FIRST_BIT: u32 = 1 << (u32::BITS - 1);
        self.data_size & (!FIRST_BIT)
    }

    /// Returns the referenced value
    pub fn value(&self) -> &RegistryValue {
        &self.value
    }

    /// Returns the datatype
    pub fn data_type(&self) -> Option<&KeyValueDataType> {
        self.data_type.as_ref()
    }
}

fn parse_value_flags<R: Read + Seek>(
    reader: &mut R,
    _ro: &ReadOptions,
    _: (),
) -> BinResult<KeyValueFlags> {
    let raw_value: u16 = reader.read_le()?;
    Ok(KeyValueFlags::from_bits_truncate(raw_value))
}

impl From<Cell<KeyValueWithMagic, ()>> for KeyValue {
    fn from(cell: Cell<KeyValueWithMagic, ()>) -> Self {
        cell.into_data().0
    }
}
