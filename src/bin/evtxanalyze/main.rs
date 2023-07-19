use clap::Parser;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::analyze::{pstree::display_pstree, Cli};

mod cli;
mod pstree;
mod sessions;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    match &cli.command {
        //TODO: move `display_pstree` into `impl Cli`
        analyze::Command::PsTree { .. } => display_pstree(&cli),
        analyze::Command::Sessions { .. } => cli.display_sessions(),
        analyze::Command::Session { .. } => cli.display_single_session(),
    }
}
