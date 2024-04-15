use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;

/// Exporter for Windows Registry Policy Files
#[derive(Parser, Debug)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, long_about = None)]
pub (crate) struct Cli {
    /// Name of the file to read
    #[clap(value_hint=ValueHint::FilePath)]
    pub (crate) polfile: String,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,

}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}