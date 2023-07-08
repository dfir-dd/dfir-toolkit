mod key_value_data;
mod key_value_data_type;
mod registry_string;

use std::io::{Read, Seek};

use binread::BinResult;
use encoding_rs::{UTF_16LE, WINDOWS_1252};
pub use key_value_data::*;
pub use key_value_data_type::*;
pub use registry_string::*;


pub fn parse_reg_sz_raw(raw_string: &[u8]) -> BinResult<String> {
    let (cow, _, had_errors) = UTF_16LE.decode(raw_string);

    if !had_errors {
        Ok(cow.strip_suffix('\0').unwrap_or(&cow).to_owned())
    } else {
        let (cow, _, had_errors) = WINDOWS_1252.decode(raw_string);
        if had_errors {
            Err(binread::error::Error::Custom {
                pos: 0,
                err: Box::new("unable to decode RegSZ string"),
            })
        } else {
            //assert_eq!(raw_string.len(), cow.len());
            Ok(cow.strip_suffix('\0').unwrap_or(&cow).to_owned())
        }
    }
}

fn read_vec<S: Read + Seek, I: TryInto<usize>>(reader: &mut S, bytes: I) -> BinResult<Vec<u8>>
where
    <I as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
{
    let mut bytes = vec![0u8; TryInto::try_into(bytes).unwrap()];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

pub fn parse_reg_multi_sz(raw_string: &[u8]) -> BinResult<Vec<String>> {
    let mut multi_string: Vec<String> = parse_reg_sz_raw(raw_string)?
        .split('\0')
        .map(|x| x.to_owned())
        .collect();

    // due to the way split() works we have an empty string after the last \0 character
    // and due to the way RegMultiSZ works we have an additional empty string between the
    // last two \0 characters.
    // those additional empty strings will be deleted afterwards:
    assert!(!multi_string.len() >= 2);
    //assert_eq!(multi_string.last().unwrap().len(), 0);
    multi_string.pop();

    if multi_string.last().is_some() && multi_string.last().unwrap().is_empty() {
        // assert_eq!(multi_string.last().unwrap().len(), 0);
        multi_string.pop();
    }

    Ok(multi_string)
}