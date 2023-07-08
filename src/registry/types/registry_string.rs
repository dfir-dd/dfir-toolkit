use binread::{BinRead};
use encoding_rs::{UTF_16LE, WINDOWS_1252};
use serde::Serialize;

#[derive(Serialize)]
pub struct RegistryString(String);

impl BinRead for RegistryString {
    type Args = usize;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _options: &binread::ReadOptions,
        args: Self::Args,
    ) -> binread::BinResult<Self> {
        let bytes = super::read_vec(reader, args)?;

        let (cow, _, had_errors) = UTF_16LE.decode(&bytes[..]);
    
        if !had_errors {
            Ok(Self(cow.strip_suffix('\0').unwrap_or(&cow).to_owned()))
        } else {
            let (cow, _, had_errors) = WINDOWS_1252.decode(&bytes[..]);
            if had_errors {
                Err(binread::error::Error::Custom {
                    pos: 0,
                    err: Box::new("unable to decode string"),
                })
            } else {
                //assert_eq!(raw_string.len(), cow.len());
                Ok(Self(cow.strip_suffix('\0').unwrap_or(&cow).to_owned()))
            }
        }
    }
}

impl From<RegistryString> for String {
    fn from(value: RegistryString) -> Self {
        value.0
    }
}

impl AsRef<str> for RegistryString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}