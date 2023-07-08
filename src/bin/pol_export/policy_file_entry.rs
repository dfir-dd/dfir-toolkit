use std::io::{Read, Seek};

use binread::{derive_binread, BinReaderExt, BinResult, ReadOptions, NullWideString};
use dfir_toolkit::registry::types::{KeyValueDataType, KeyValueData};
use serde::Serialize;

/// https://learn.microsoft.com/en-us/previous-versions/windows/desktop/policy/registry-policy-file-format
#[derive(Serialize)]
#[derive_binread]
#[br(little)]
pub struct PolicyFileEntry {
    #[br(assert(_begin == '['), parse_with=read_char)]
    #[serde(skip_serializing)]
    _begin: char,

    #[br(parse_with=parse_wide_string)]
    key: String,

    #[br(assert(_sep1 == ';'), parse_with=read_char)]
    #[serde(skip_serializing)]
    _sep1: char,

    #[br(parse_with=parse_wide_string)]
    value_name: String,

    #[br(assert(_sep2 == ';'), parse_with=read_char)]
    #[serde(skip_serializing)]
    _sep2: char,

    value_type: KeyValueDataType,

    #[br(assert(_sep3 == ';'), parse_with=read_char)]
    #[serde(skip_serializing)]
    _sep3: char,

    #[serde(skip_serializing)]
    #[br(temp)]    
    size: u32,

    #[br(assert(_sep4 == ';'), parse_with=read_char)]
    #[serde(skip_serializing)]
    _sep4: char,

    #[br(args(value_type, size))]
    value_data: KeyValueData,

    #[br(assert(_end == ']'), parse_with=read_char)]
    #[serde(skip_serializing)]
    _end: char,
}

fn parse_wide_string<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _args: ()) -> BinResult<String> {
    let ws: NullWideString = reader.read_le()?;
    Ok(ws.into_string())
}

fn read_char<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _args: ()) -> BinResult<char> {
    let b: [u16; 1] = reader.read_le()?;
    Ok(char::decode_utf16(b.into_iter())
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .next()
        .unwrap())
}