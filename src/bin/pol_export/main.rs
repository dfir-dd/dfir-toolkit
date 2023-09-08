use std::{
    fs::File,
    io::{stdout, Read},
};

use anyhow::{anyhow, Result};
use binread::{BinReaderExt, BinResult};
use cli::Cli;
use dfir_toolkit::common::FancyParser;

mod cli;
mod policy_file_entry;

use csv::Writer;
use policy_file_entry::*;

fn main() -> Result<()> {
    let args = Cli::parse_cli();

    let mut polfile = File::open(args.polfile)?;

    let mut header: [u8; 4] = [0; 4];
    polfile.read_exact(&mut header)?;
    if &header != b"PReg" {
        return Err(anyhow!("invalid magic number"));
    }

    let mut version: [u8; 4] = [0; 4];
    polfile.read_exact(&mut version)?;
    if &version != b"\x01\0\0\0" {
        return Err(anyhow!("invalid version number"));
    }

    let mut wtr = Writer::from_writer(stdout());

    loop {
        let entry_result: BinResult<PolicyFileEntry> = polfile.read_le();
        match entry_result {
            Ok(entry) => {
                wtr.serialize(entry)?;
            }
            Err(why) => match why {
                binread::Error::Io(why) if why.kind() == std::io::ErrorKind::OutOfMemory => break,
                binread::Error::Io(why) if why.kind() == std::io::ErrorKind::BrokenPipe => break,
                binread::Error::Io(why) if why.kind() == std::io::ErrorKind::UnexpectedEof => break,
                _ => {
                    log::error!("{why}");
                    //continue;
                    break;
                }
            },
        }
    }
    Ok(())
}
