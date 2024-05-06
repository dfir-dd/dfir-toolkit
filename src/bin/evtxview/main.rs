mod cli;
mod system_field;
mod ui_main;
mod evtx_line;
mod evtx_column;
mod evtx_view;

use anyhow::{bail, Result};
use cli::Cli;
use clap::Parser;
use ui_main::UIMain;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if ! cli.evtx_file.path().exists() {
        bail!("invalid filename specified: file does not exist");
    }

    if ! cli.evtx_file.path().is_file() {
        bail!("invalid filename specified: filename does not point to a file");
    }

    cursive::logger::init();

    let mut ui = UIMain::new(cli.evtx_file.path().path())?;
    ui.run()
}
