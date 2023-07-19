use clap::Parser;
use cli::{Cli, Command};
use pstree::display_pstree;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

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
        Command::PsTree { .. } => display_pstree(&cli),
        Command::Sessions { .. } => cli.display_sessions(),
        Command::Session { .. } => cli.display_single_session(),
    }
}
