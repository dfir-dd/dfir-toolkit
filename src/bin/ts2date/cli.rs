use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;
use clio::{Input,Output};

/// replaces UNIX timestamps in a stream by a formatted date 
#[derive(Parser, Debug)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, long_about = None)]
pub (crate) struct Cli {
    /// name of the file to read (default from stdin)
    #[clap(value_parser, value_hint=ValueHint::FilePath, default_value="-", display_order(100))]
    pub(crate) input_file: Input,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,

    /// name of the file to write (default to stdout)
    #[clap(default_value="-", value_hint=ValueHint::FilePath, value_parser)]
    pub(crate) output_file: Output,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}