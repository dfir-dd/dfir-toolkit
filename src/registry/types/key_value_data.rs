use std::{
    fmt::Display,
};

use binread::{BinRead, BinReaderExt};
use chrono::{DateTime, Utc};
use serde::Serialize;
use winstructs::timestamp::WinTimestamp;

use super::{KeyValueDataType, RegistryString};

/// Represents a binary registry datum
///
/// ```
/// # use std::io::Cursor;
/// # use std::io::Read;
/// # use binread::BinReaderExt;
/// # use windows_types_registry::*;
///
/// let mut reader = Cursor::new(b"Test\0");
/// let parsed_data: KeyValueData = reader.read_ne_args((KeyValueDataType::RegSZ, 5)).unwrap();
/// assert_eq!(parsed_data, KeyValueData::RegSZ("Test".to_string()));
/// ```
#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum KeyValueData {
    RegNone,
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
    RegFileTime(DateTime<Utc>),
}

impl BinRead for KeyValueData {
    type Args = (KeyValueDataType, u32);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let data_type = &args.0;
        let data_size: usize = TryInto::try_into(args.1).unwrap();

        Ok(match data_type {
            KeyValueDataType::RegNone => Self::RegNone,
            KeyValueDataType::RegSZ => Self::RegSZ(reader.read_le_args::<RegistryString>(data_size)?.into()),
            KeyValueDataType::RegExpandSZ => {
                Self::RegExpandSZ(reader.read_le_args::<RegistryString>(data_size)?.into())
            }
            KeyValueDataType::RegBinary => {
                let mut bytes = vec![0u8; data_size];
                reader.read_exact(&mut bytes)?;
                Self::RegBinary(bytes)
            }
            KeyValueDataType::RegDWord => {
                if data_size != 4 {
                    return Err(binread::Error::AssertFail {
                        pos: options.offset,
                        message: "RegDWord must be exactly 4 bytes large".to_string(),
                    });
                }
                Self::RegDWord(reader.read_le()?)
            }
            KeyValueDataType::RegDWordBigEndian => {
                if data_size != 4 {
                    return Err(binread::Error::AssertFail {
                        pos: options.offset,
                        message: "RegDWordBigEndian must be exactly 4 bytes large".to_string(),
                    });
                }
                Self::RegDWordBigEndian(reader.read_be()?)
            }
            KeyValueDataType::RegLink => Self::RegLink(reader.read_le_args::<RegistryString>(data_size)?.into()),
            KeyValueDataType::RegMultiSZ => {
                let bytes = super::read_vec(reader, data_size)?;
                let strings = super::parse_reg_multi_sz(&bytes[..])?;
                Self::RegMultiSZ(strings)
            }
            KeyValueDataType::RegResourceList => {
                Self::RegResourceList(reader.read_le_args::<RegistryString>(data_size)?.into())
            }
            KeyValueDataType::RegFullResourceDescriptor => {
                Self::RegFullResourceDescriptor(reader.read_le_args::<RegistryString>(data_size)?.into())
            }
            KeyValueDataType::RegResourceRequirementsList => {
                Self::RegResourceRequirementsList(reader.read_le_args::<RegistryString>(data_size)?.into())
            }
            KeyValueDataType::RegQWord => {
                if data_size != 8 {
                    return Err(binread::Error::AssertFail {
                        pos: options.offset,
                        message: "RegQWord must be exactly 8 bytes large".to_string(),
                    });
                }
                Self::RegQWord(reader.read_le()?)
            }
            KeyValueDataType::RegFileTime => {
                if data_size != 8 {
                    return Err(binread::Error::AssertFail {
                        pos: options.offset,
                        message: "RegFileTime must be exactly 8 bytes large".to_string(),
                    });
                }
                let ft = match WinTimestamp::from_reader(reader) {
                    Ok(ts) => ts,
                    Err(why) => match why {
                        winstructs::err::Error::IoError { source } => {
                            return Err(binread::Error::Io(source))
                        }
                        winstructs::err::Error::UnknownAceType { ace_type: _ } => {
                            return Err(binread::Error::Custom {
                                pos: options.offset,
                                err: Box::new(why),
                            })
                        }
                    },
                };
                Self::RegFileTime(ft.to_datetime())
            }
        })
    }
}

impl Display for KeyValueData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyValueData::RegNone => write!(f, "None"),
            KeyValueData::RegSZ(val) => write!(f, "{val:?}"),
            KeyValueData::RegExpandSZ(val) => write!(f, "{val:?}"),
            KeyValueData::RegBinary(val) => {
                write!(f, "{:?}", if val.len() > 16 { &val[..16] } else { val })
            }
            KeyValueData::RegDWord(val) => write!(f, "0x{:08x}", val),
            KeyValueData::RegDWordBigEndian(val) => write!(f, "0x{:08x}", val),
            KeyValueData::RegLink(val) => write!(f, "{val:?}"),
            KeyValueData::RegMultiSZ(val) => write!(f, "{:?}", val),
            KeyValueData::RegResourceList(val) => write!(f, "{val:?}"),
            KeyValueData::RegFullResourceDescriptor(val) => write!(f, "{val:?}"),
            KeyValueData::RegResourceRequirementsList(val) => write!(f, "{val:?}"),
            KeyValueData::RegQWord(val) => write!(f, "0x{:016x}", val),
            KeyValueData::RegFileTime(dt) => write!(f, "{datetime}", datetime = dt.to_rfc3339()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binread::BinReaderExt;
    use chrono::{Duration, NaiveDate};
    use std::io::Cursor;

    #[test]
    fn test_parse_sz_cp1252_ok() {
        let mut reader = Cursor::new(b"Test\0");
        let parsed_data: KeyValueData = reader.read_ne_args((KeyValueDataType::RegSZ, 5)).unwrap();
        assert_eq!(parsed_data, KeyValueData::RegSZ("Test".to_string()));
    }

    #[test]
    fn test_parse_sz_utf16_ok() {
        let mut reader = Cursor::new(b"T\0e\0s\0t\0\0\0");
        let parsed_data: KeyValueData = reader.read_ne_args((KeyValueDataType::RegSZ, 10)).unwrap();
        assert_eq!(parsed_data, KeyValueData::RegSZ("Test".to_string()));
    }

    #[test]
    fn test_parse_dword_ok() {
        let mut reader = Cursor::new(b"\x01\x02\x03\x04");
        let parsed_data: KeyValueData = reader
            .read_ne_args((KeyValueDataType::RegDWord, 4))
            .unwrap();
        assert_eq!(parsed_data, KeyValueData::RegDWord(0x04030201));
    }

    #[test]
    fn test_parse_dword_big_endian_ok() {
        let mut reader = Cursor::new(b"\x01\x02\x03\x04");
        let parsed_data: KeyValueData = reader
            .read_ne_args((KeyValueDataType::RegDWordBigEndian, 4))
            .unwrap();
        assert_eq!(parsed_data, KeyValueData::RegDWordBigEndian(0x01020304));
    }

    #[test]
    fn test_parse_qword_ok() {
        let mut reader = Cursor::new(b"\x01\x02\x03\x04\x05\x06\x07\x08");
        let parsed_data: KeyValueData = reader
            .read_ne_args((KeyValueDataType::RegQWord, 8))
            .unwrap();
        assert_eq!(parsed_data, KeyValueData::RegQWord(0x0807060504030201));
    }

    #[test]
    fn test_parse_filetime_ok() {
        let mut reader = Cursor::new(b"\x66\x47\x46\x20\x77\xDE\xCF\x01");
        let parsed_data: KeyValueData = reader
            .read_ne_args((KeyValueDataType::RegFileTime, 8))
            .unwrap();

        let expected = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd_opt(2014, 10, 2)
                .unwrap()
                .and_hms_opt(19, 29, 4)
                .unwrap(),
            Utc,
        );
        let nanos = Duration::microseconds(98493);
        assert_eq!(parsed_data, KeyValueData::RegFileTime(expected + nanos));
    }

    #[test]
    fn test_parse_multi_sz_cp1252_ok() {
        let bytes = b"Test1\0Test2\0Test3\0\0";
        let mut reader = Cursor::new(bytes);
        let parsed_data: KeyValueData = reader
            .read_ne_args((KeyValueDataType::RegMultiSZ, bytes.len() as u32))
            .unwrap();
        assert_eq!(
            parsed_data,
            KeyValueData::RegMultiSZ(vec![
                "Test1".to_string(),
                "Test2".to_string(),
                "Test3".to_string()
            ])
        );
    }
}
