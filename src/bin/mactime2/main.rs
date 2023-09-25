use anyhow::Result;
use dfir_toolkit::common::{FancyParser, TzArgument};

use dfir_toolkit::apps::mactime2::Cli;
use dfir_toolkit::apps::mactime2::Mactime2Application;

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
