use std::io::Stdout;

use anyhow::{bail, Result};
use cli::Cli;
use dfir_toolkit::common::FancyParser;
use evtx_file::EvtxFile;
use output_format::OutputFormat;
use output_writer::{BodyfileOutputWriter, RecordOutputWriter};

mod bf_data;
mod cli;
mod evtx_file;
mod output_format;
mod output_writer;
mod value_map;
#[macro_use]
mod macros;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    if cli.evtx_files().iter().any(|f| !f.can_seek()) {
        bail!(
            "{} cannot read from a stream; you must specify a file",
            env!("CARGO_BIN_NAME")
        );
    }

    for input in cli.evtx_files().iter() {
        let file = EvtxFile::from(input);

        match cli.format() {
            OutputFormat::Bodyfile => file.print_records::<BodyfileOutputWriter<Stdout>>(!cli.strict())?,
            OutputFormat::Record => file.print_records::<RecordOutputWriter<Stdout>>(!cli.strict())?,
        }
    }

    Ok(())
}
