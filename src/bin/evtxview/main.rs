mod app;
mod cli;
mod tui;
mod event;

use anyhow::Result;
use app::App;
use cli::Cli;
use dfir_toolkit::common::FancyParser;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();
    let mut terminal = tui::init()?;
    let app_result = App::new(cli).run(&mut terminal);
    tui::restore(&mut terminal)?;
    Ok(app_result?)
}
