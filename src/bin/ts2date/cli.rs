use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;
use clio::Input;


#[derive(Parser, Debug)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, about, long_about = None)]
pub (crate) struct Cli {
    /// Name of the file to read
    #[clap(value_parser, value_hint=ValueHint::FilePath, default_value="-", help="if no file inputed, read from stdin", display_order(100))]
    pub(crate) input_file: Input,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,

}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}