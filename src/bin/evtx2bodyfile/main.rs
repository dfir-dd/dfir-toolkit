use anyhow::Result;
use evtx2bodyfile_app::Evtx2BodyfileApp;
use dfir_toolkit::common::FancyParser;

mod bf_data;
mod evtx2bodyfile_app;
mod evtx_file;
#[macro_use]
mod macros;

fn main() -> Result<()> {
    if std::env::args().any(|a| &a == "--markdown-help") {
        clap_markdown::print_help_markdown::<Evtx2BodyfileApp>();
        return Ok(());
    }
    let cli = Evtx2BodyfileApp::parse_cli();

    cli.handle_evtx_files()
}
