use clap::Parser;
use clap::ValueHint;
use clio::ClioPath;
use dfir_toolkit::common::HasVerboseFlag;
use getset::Getters;
use log::LevelFilter;

/// creates bodyfile from ZIP Archives based on the contained files and folders
#[derive(Parser, Getters)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
#[getset(get = "pub (crate)")]
pub(crate) struct Cli {
    /// names of the archive files (commonly files with 'zip' extension)
    #[clap(value_hint=ValueHint::FilePath)]
    zip_files: Vec<ClioPath>,

    /// show the name of the archive in the bodyfile output
    #[clap(long("show-archive-name"))]
    show_archive_name: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}
