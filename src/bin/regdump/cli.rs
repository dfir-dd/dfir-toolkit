use std::{path::PathBuf, fs::File};

use clap::{Parser, ValueHint};
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;
use nt_hive2::{HiveParseMode, Hive};

/// parses registry hive files and prints a bodyfile
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, long_about = None)]
pub (crate) struct Cli {
    /// name of the file to dump
    #[arg(value_parser = validate_file, value_hint=ValueHint::FilePath)]
    pub(crate) hive_file: PathBuf,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[clap(short('L'), long("log"), value_hint=ValueHint::FilePath)]
    #[arg(value_parser = validate_file)]
    pub (crate) logfiles: Vec<PathBuf>,

    /// print as bodyfile format
    #[clap(short('b'), long("bodyfile"))]
    pub (crate) display_bodyfile: bool,

    /// ignore the base block (e.g. if it was encrypted by some ransomware)
    #[clap(short('I'), long)]
    pub (crate) ignore_base_block: bool,

    /// hide timestamps, if output is in reg format
    #[clap(short('T'), long)]
    pub (crate) hide_timestamps: bool,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub fn parse_mode(&self) -> HiveParseMode {
        if self.ignore_base_block {
            match File::open(&self.hive_file) {
                Ok(data) => {
                    let hive = Hive::new(data, HiveParseMode::Raw).unwrap();
                    let offset = match hive.find_root_celloffset() {
                        Some(offset) => offset,
                        None => {
                            log::error!("scan found no root cell offset, aborting...");
                            std::process::exit(-1);
                        }
                    };
                    println!("found offset at {}", offset.0);
                    HiveParseMode::Normal(offset)
                }
                Err(why) => {
                    log::error!(
                        "unable to open '{}': {}",
                        self.hive_file.to_string_lossy(),
                        why
                    );
                    std::process::exit(-1);
                }
            }
        } else {
            HiveParseMode::NormalWithBaseBlock
        }
    }
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
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
