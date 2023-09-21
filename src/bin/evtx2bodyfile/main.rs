use anyhow::Result;
use cli::Cli;
use dfir_toolkit::common::FancyParser;

mod bf_data;
mod cli;
mod evtx_file;
#[macro_use]
mod macros;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    cli.handle_evtx_files()
}
