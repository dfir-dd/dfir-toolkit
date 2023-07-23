use std::{io::{Read, Seek}};

use binread::{ReadOptions, BinResult, BinRead, BinReaderExt};
use chrono::{DateTime, Utc};
use encoding_rs::{ISO_8859_15, UTF_16LE};
use winstructs::timestamp::WinTimestamp;

pub (crate) fn parse_string<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, params: (bool,))
-> BinResult<String> {
    let raw_string = Vec::<u8>::read_options(reader, ro, ())?;

    let (cow, _, had_errors) = 
    if params.0 {
        ISO_8859_15.decode(&raw_string[..])
    } else {
        UTF_16LE.decode(&raw_string[..])
    };

    if had_errors {
        Err(binread::error::Error::Custom {
            pos: ro.offset,
            err: Box::new(format!("unable to decode String at offset 0x{:08x}", ro.offset))})
    } else {
        Ok(cow.to_string())
    }
}

pub (crate) fn parse_reg_sz(raw_string: &[u8]) -> BinResult<String> {
    let res = parse_reg_sz_raw(raw_string)?;
    Ok(res.trim_end_matches(char::from(0)).to_string())
}

pub fn parse_reg_sz_raw(raw_string: &[u8]) -> BinResult<String> {
    let (cow, _, had_errors) = UTF_16LE.decode(raw_string);
    if ! had_errors {
        Ok(cow.to_string())
    } else {

        let (cow, _, had_errors) = ISO_8859_15.decode(raw_string);
        if had_errors {
            Err(binread::error::Error::Custom {
                pos: 0,
                err: Box::new("unable to decode RegSZ string")})
        } else {
            //assert_eq!(raw_string.len(), cow.len());
            Ok(cow.to_string())
        }
    }
}

pub (crate) fn parse_reg_multi_sz(raw_string: &[u8]) -> BinResult<Vec<String>> {
    let mut multi_string: Vec<String> = parse_reg_sz_raw(raw_string)?.split('\0')
        .map(|x| x.to_owned())
        .collect();
    
    // due to the way split() works we have an empty string after the last \0 character
    // and due to the way RegMultiSZ works we have an additional empty string between the
    // last two \0 characters.
    // those additional empty strings will be deleted afterwards:
    assert!(! multi_string.len() >= 2);
    //assert_eq!(multi_string.last().unwrap().len(), 0);
    multi_string.pop();

    if multi_string.last().is_some() {
        // assert_eq!(multi_string.last().unwrap().len(), 0);
        multi_string.pop();
    }

    Ok(multi_string)
}

pub (crate) fn parse_timestamp<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ())
-> BinResult<DateTime<Utc>> {
    let raw_timestamp: [u8;8] = reader.read_le()?;
    let timestamp = WinTimestamp::new(&raw_timestamp).unwrap();
    Ok(timestamp.to_datetime())
}

pub const U32_FIRST_BIT: u32 = 1 << (u32::BITS - 1);
pub const INV_U32_FIRST_BIT: u32 = ! (1 << (u32::BITS - 1));
pub (crate) trait HasFirstBitSet {
    fn has_first_bit_set (val: &Self) -> bool;
}

impl HasFirstBitSet for u32 {
    fn has_first_bit_set (val: &Self) -> bool {
        val & U32_FIRST_BIT == U32_FIRST_BIT
    }
}

pub(crate) const BIG_DATA_SEGMENT_SIZE: u32 = 16344;
pub (crate) fn without_first_bit(val: u32) -> u32 {
    val & INV_U32_FIRST_BIT
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_first_bit_set () {
        assert!(u32::has_first_bit_set(& 0x8000_0000));
        assert!(u32::has_first_bit_set(& 0xFFFF_FFFF));
        assert!(! u32::has_first_bit_set(& 0x7FFF_FFFF));
        assert!(! u32::has_first_bit_set(& 0));
        assert!(! u32::has_first_bit_set(& 1));
        assert!(! u32::has_first_bit_set(& (i32::MAX as u32)));
    }

    #[test]
    fn test_without_first_bit() {
        assert_eq!(0, without_first_bit(0));
        assert_eq!(1, without_first_bit(1));
        assert_eq!((i32::MAX as u32), without_first_bit(i32::MAX as u32));

        assert_eq!(8, without_first_bit(0x8000_0008));
        assert_eq!(16, without_first_bit(0x8000_0010));
        assert_eq!(32, without_first_bit(0x8000_0020));
    }
}