use std::path::PathBuf;

use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;


/// scans a registry hive file for deleted entries
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
pub (crate) struct Cli {
    /// name of the file to scan
    #[clap(value_hint=ValueHint::FilePath)]
    pub (crate) hive_file: String,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[clap(short('L'), long("log"), value_hint=ValueHint::FilePath)]
    #[arg(value_parser = validate_file)]
    pub (crate) logfiles: Vec<PathBuf>,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,

    /// output as bodyfile format
    #[clap(short('b'))]
    pub (crate) print_bodyfile: bool,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self)-> LevelFilter {
        self.verbose.log_level_filter()
    }
}

fn validate_file(s: &str) -> Result<PathBuf, String> {
    let pb = PathBuf::from(s);
    if pb.is_file() && pb.exists() {
        Ok(pb)
    } else {
        Err(format!("unable to read file: '{s}'"))
    }
}