use anyhow::Result;
use cli::Cli;
use colored::control::SHOULD_COLORIZE;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::{fs::File, io::BufReader};

use dfir_toolkit::common::FancyParser;

mod cli;
mod format_ipv4;
mod ip_filter;
mod ipv4_with_properties;
use crate::format_ipv4::format_ipv4;

fn main() -> Result<()> {
    let app = Cli::parse_cli();

    TermLogger::init(
        app.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    if app.display_colors {
        SHOULD_COLORIZE.set_override(true);
    }

    if app.file.is_empty() {
        app.ipgrep(std::io::stdin().lock())?;
    } else {
        for file in app.file.iter() {
            let f = File::open(file)?;
            app.ipgrep(BufReader::new(f))?;
        }
    }

    Ok(())
}
