use cli::{Cli, Command};
use dfir_toolkit::common::FancyParser;
use log::log_enabled;
use pstree::display_pstree;

mod cli;
mod pstree;
mod sessions;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_cli();

    let result = match &cli.command {
        //TODO: move `display_pstree` into `impl Cli`
        Command::PsTree { .. } => display_pstree(&cli),
        Command::Sessions { .. } => cli.display_sessions(),
        Command::Session { .. } => cli.display_single_session(),
    };

    if let Err(why) = result {
        log::error!("{why}");
        if let Some(cause) = why.source() {
            log::error!("caused by: {cause}");
        }
        if log_enabled!(log::Level::Warn) {
            for line in format!("{}", why.backtrace()).lines() {
                log::warn!("{line}");
            }
        }
        std::process::exit(exitcode::DATAERR);
    }

    Ok(())
}
