use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;
use clio::{Input,Output};

/// Parse Windows LNK files and create bodyfile output
#[derive(Parser, Debug)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, long_about = None)]
pub (crate) struct Cli {
    /// Name of the LNK files to read from
    #[clap(value_parser, value_hint=ValueHint::FilePath, default_value="-", display_order(100))]
    pub(crate) lnk_files: Vec<Input>,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}