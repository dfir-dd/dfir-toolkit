use anyhow::Result;
use chrono_tz::TZ_VARIANTS;
use dfir_toolkit::common::FancyParser;

use dfir_toolkit::apps::mactime2::Cli;
use dfir_toolkit::apps::mactime2::Mactime2Application;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse_cli();

    match cli.src_zone().as_deref() {
        Some("list") => {display_zones(); return Ok(());}
        Some(_) => {}
        _ => {}
    }

    match cli.dst_zone().as_deref() {
        Some("list") => {display_zones(); return Ok(());}
        Some(_) => {}
        _ => {}
    }

    let app: Mactime2Application = cli.into();

    app.run()
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
}
