use dfir_toolkit::common::FancyParser;
use cli::Cli;
use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    Ok(())
}