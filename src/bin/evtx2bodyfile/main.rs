use anyhow::{Result, bail};
use cli::Cli;
use dfir_toolkit::common::FancyParser;

mod bf_data;
mod cli;
mod evtx_file;
#[macro_use]
mod macros;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    if cli.evtx_files().iter().any(|f| !f.can_seek()) {
        bail!("{} cannot read from a stream; you must specify a file", env!("CARGO_BIN_NAME"));
    }

    cli.handle_evtx_files()
}
