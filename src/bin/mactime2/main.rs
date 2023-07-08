use anyhow::Result;
use clap::Parser;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};
use chrono_tz::TZ_VARIANTS;

use dfir_toolkit::apps::mactime2::Cli;
use dfir_toolkit::apps::mactime2::Mactime2Application;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let _ = TermLogger::init(
        cli.verbose().log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);

    let _list = "list".to_owned();
    if matches!(cli.src_zone(), Some(_list)) {
        display_zones(); return Ok(());
    }
    if matches!(cli.dst_zone(), Some(_list)) {
        display_zones(); return Ok(());
    }

    let app: Mactime2Application = cli.into();

    app.run()
}

fn display_zones() {
    for v in TZ_VARIANTS.iter() {
        println!("{}", v);
    }
}
