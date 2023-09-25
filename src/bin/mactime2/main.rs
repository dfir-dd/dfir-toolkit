mod application;
mod stream;
mod bodyfile;
mod error;
mod filter;
mod output;
mod cli;

use application::*;
use cli::*;

use anyhow::Result;
use dfir_toolkit::common::{FancyParser, TzArgument};

fn main() -> Result<()> {
    let cli: Cli = Cli::parse_cli();

    if cli.src_zone.is_list() || cli.dst_zone.is_list() {
        TzArgument::display_zones();
        return Ok(());
    }
    debug_assert!(cli.dst_zone.is_tz());
    debug_assert!(cli.src_zone.is_tz());

    let app: Mactime2Application = cli.into();

    app.run()
}
