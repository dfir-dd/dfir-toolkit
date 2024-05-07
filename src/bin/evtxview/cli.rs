use clap::{Parser, ValueEnum, ValueHint};

use clio::InputPath;
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;

#[derive(ValueEnum, Clone)]
pub(crate) enum SortOrder {
    /// don't change order, output records as they are stored
    Storage,

    /// sort by event record id
    RecordId,

    /// sort by date and time
    Time,
}

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version,long_about=None)]
pub struct Cli {
    /// Name of the evtx files to read from
    #[clap(value_hint=ValueHint::FilePath)]
    pub(crate) evtx_file: InputPath,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}
