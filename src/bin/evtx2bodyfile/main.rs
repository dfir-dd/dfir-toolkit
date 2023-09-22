use anyhow::{bail, Result};
use cli::Cli;
use dfir_toolkit::common::FancyParser;
use evtx_file::EvtxFile;
use output_formatter::{BodyfileOutputFormatter, JsonOutputFormatter};

mod bf_data;
mod cli;
mod evtx_file;
mod output_format;
mod output_formatter;
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
            output_format::OutputFormat::Json => {
                file.print_records(JsonOutputFormatter, !cli.strict())?
            }
            output_format::OutputFormat::Bodyfile => {
                file.print_records(BodyfileOutputFormatter, !cli.strict())?
            }
        }
    }
    Ok(())
}
