use std::{fs::File, path::PathBuf};

use anyhow::{bail, Result};
use clap::{Parser, ValueHint};
use dfir_toolkit::common::FancyParser;
use nt_hive2::{ContainsHive, Hive, HiveParseMode};
use simplelog::{Config, SimpleLogger};

/// merges logfiles into a hive file
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
struct Args {
    /// name of the file to dump
    #[arg(num_args=1, value_hint=ValueHint::FilePath)]
    pub(crate) hive_file: String,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[arg(short('L'), long("log"), num_args=0.., value_parser = validate_file, value_hint=ValueHint::FilePath)]
    logfiles: Vec<PathBuf>,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,

    /// name of the file to which the cleaned hive will be written.
    #[arg(short('O'), long("output"), num_args=1, value_hint=ValueHint::FilePath)]
    pub(crate) dst_hive: String,
}

fn validate_file(s: &str) -> Result<PathBuf, String> {
    let pb = PathBuf::from(s);
    if pb.is_file() && pb.exists() {
        Ok(pb)
    } else {
        Err(format!("unable to read file: '{s}'"))
    }
}

pub fn main() -> Result<()> {
    let mut cli = Args::parse_cli();

    let _ = SimpleLogger::init(cli.verbose.log_level_filter(), Config::default());

    let hive_file = PathBuf::from(&cli.hive_file);
    if !hive_file.exists() {
        bail!("missing hive file: {}", cli.hive_file);
    }

    let hive_file = File::open(hive_file)?;
    let hive = Hive::new(hive_file, HiveParseMode::NormalWithBaseBlock).unwrap();

    let mut clean_hive = match cli.logfiles.len() {
        0 => {
            log::warn!("no log files provided, treating hive as if it was clean");
            hive.treat_hive_as_clean()
        }
        1 => hive
            .with_transaction_log(File::open(cli.logfiles.pop().unwrap())?.try_into()?)?
            .apply_logs(),
        2 => hive
            .with_transaction_log(File::open(cli.logfiles.pop().unwrap())?.try_into()?)?
            .with_transaction_log(File::open(cli.logfiles.pop().unwrap())?.try_into()?)?
            .apply_logs(),
        _ => {
            bail!("more than two transaction log files are not supported")
        }
    };

    let mut dst = File::create(cli.dst_hive)?;
    std::io::copy(&mut clean_hive, &mut dst)?;
    Ok(())
}
