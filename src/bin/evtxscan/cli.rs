use clap::Parser;
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;


/// Find time skews in an evtx file
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
pub (crate) struct Cli {
    /// name of the evtx file to scan
    pub (crate) evtx_file: String,

    /// display also the contents of the records befor and after a time skew
    #[clap(short = 'S', long)]
    pub (crate) show_records: bool,

    /// negative tolerance limit (in seconds): time skews to the past below this limit will be ignored
    #[clap(short = 'N', long, default_value_t = 5)]
    pub (crate) negative_tolerance: u32,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self)-> LevelFilter {
        self.verbose.log_level_filter()
    }
}