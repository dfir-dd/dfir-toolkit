use crate::output_format::OutputFormat;
use clap::{Parser, ValueHint};
use clio::Input;
use dfir_toolkit::common::HasVerboseFlag;
use getset::Getters;
use log::LevelFilter;

#[derive(Parser, Clone, Getters)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, about, long_about = None)]
#[getset(get = "pub (crate)")]
pub(crate) struct Cli {
    /// names of the evtx files
    #[clap(value_hint=ValueHint::FilePath)]
    evtx_files: Vec<Input>,

    /// select output format
    #[clap(short('F'), long("format"), default_value_t=OutputFormat::Bodyfile)]
    format: OutputFormat,

    /// fail upon read error
    #[clap(short('S'), long("strict"))]
    strict: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}
