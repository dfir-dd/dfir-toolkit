use clap::Parser;
use clap::ValueHint;
use clio::ClioPath;
use dfir_toolkit::common::HasVerboseFlag;
use getset::Getters;
use log::LevelFilter;

/// creates bodyfile from Windows Prefetch files
#[derive(Parser, Getters)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
#[getset(get = "pub (crate)")]
pub(crate) struct Cli {
    /// names of the prefetch files (commonly files with 'pf' extension in 'C:\Windows\Prefetch')
    #[clap(value_hint=ValueHint::FilePath)]
    prefetch_files: Vec<ClioPath>,

    /// show not only the executed files, but all references files -- such as libraries -- as well
    #[clap(short='I')]
    include_metrics: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}
