use cli::{Cli, Command};
use pstree::display_pstree;
use dfir_toolkit::common::FancyParser;

mod cli;
mod pstree;
mod sessions;

fn main() -> anyhow::Result<()> {
    if std::env::args().any(|a| &a == "--markdown-help") {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }
    let cli = Cli::parse_cli();

    match &cli.command {
        //TODO: move `display_pstree` into `impl Cli`
        Command::PsTree { .. } => display_pstree(&cli),
        Command::Sessions { .. } => cli.display_sessions(),
        Command::Session { .. } => cli.display_single_session(),
    }
}
