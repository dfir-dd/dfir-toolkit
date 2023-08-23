use anyhow::Result;
use chrono_tz::TZ_VARIANTS;
use dfir_toolkit::common::FancyParser;

use dfir_toolkit::apps::mactime2::Cli;
use dfir_toolkit::apps::mactime2::Mactime2Application;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse_cli();

    if cli.src_zone.is_list() || cli.dst_zone.is_list() {
        display_zones();
        return Ok(());
    }
    debug_assert!(cli.dst_zone.is_tz());
    debug_assert!(cli.src_zone.is_tz());

    let app: Mactime2Application = cli.into();

    app.run()
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
}
