use clap::Parser;
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;

use crate::output_format::OutputFormat;

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"),author,version,about)]
pub (crate) struct Cli {
    /// Name of the evtx file to read from
    pub (crate) evtx_file: String,

    /// filter: minimal event record identifier
    #[clap(long)]
    pub (crate) min: Option<u64>,

    /// filter: maximal event record identifier
    #[clap(long)]
    pub (crate) max: Option<u64>,

    /// show only the one event with this record identifier
    #[clap(short, long)]
    pub (crate) id: Option<u64>,

    /// don't display the records in a table format
    #[clap(short('T'), long("display-table"))]
    pub (crate) show_table: bool,

    /// output format
    #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Xml)]
    pub (crate) format: OutputFormat,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self)-> LevelFilter {
        self.verbose.log_level_filter()
    }
}