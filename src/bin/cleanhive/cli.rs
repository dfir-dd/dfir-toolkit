use clap::{Parser, ValueHint};
use clio::*;
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;

/// merges logfiles into a hive file
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
pub (crate) struct Cli {
    /// name of the file to dump
    #[clap(num_args=1, value_parser, value_hint=ValueHint::FilePath)]
    pub(crate) hive_file: Input,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[clap(short('L'), long("log"), num_args=0.., value_parser, value_hint=ValueHint::FilePath)]
    pub(crate) logfiles: Vec<InputPath>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// name of the file to which the cleaned hive will be written.
    #[clap(short('O'), long("output"), default_value="-", num_args=1, value_hint=ValueHint::FilePath, value_parser)]
    pub(crate) dst_hive: Output,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self)-> LevelFilter {
        self.verbose.log_level_filter()
    }
}
