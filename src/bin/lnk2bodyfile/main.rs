use dfir_toolkit::common::FancyParser;
use cli::Cli;
use anyhow::{Result, bail};

use crate::lnk_file::LnkFile;

mod cli;
mod lnk_file;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    if cli.lnk_files.iter().any(|f| !f.can_seek()) {
        bail!(
            "{} cannot read from a stream; you must specify a file",
            env!("CARGO_BIN_NAME")
        );
    }

    for filename in cli.lnk_files.iter() {
        let lnkfile = match LnkFile::try_from(filename) {
            Ok(file) => file,
            Err(why) => {log::error!("{why}"); continue;},
        };
        lnkfile.print_bodyfile();
    }

    Ok(())
}