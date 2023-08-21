use anyhow::{bail, Result};
use clap::Parser;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use nt_hive2::*;
use simplelog::{Config, SimpleLogger};
use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, about, long_about = None)]
struct Args {
    /// name of the file to dump
    #[arg(value_parser = validate_file)]
    pub(crate) hive_file: PathBuf,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[clap(short('L'), long("log"))]
    #[arg(value_parser = validate_file)]
    logfiles: Vec<PathBuf>,

    /// print as bodyfile format
    #[clap(short('b'), long("bodyfile"))]
    display_bodyfile: bool,

    /// ignore the base block (e.g. if it was encrypted by some ransomware)
    #[clap(short('I'), long)]
    ignore_base_block: bool,

    /// hide timestamps, if output is in reg format
    #[clap(short('T'), long)]
    hide_timestamps: bool,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,

    /// print help in markdown format
    #[arg(long, hide = true, exclusive=true)]
    pub markdown_help: bool,
}

impl Args {
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

fn validate_file(s: &str) -> Result<PathBuf, String> {
    let pb = PathBuf::from(s);
    if pb.is_file() && pb.exists() {
        Ok(pb)
    } else {
        Err(format!("unable to read file: '{s}'"))
    }
}

fn main() -> Result<()> {
    if std::env::args().any(|a| &a == "--markdown-help") {
        clap_markdown::print_help_markdown::<Args>();
        return Ok(());
    }
    let mut cli = Args::parse();
    let _ = SimpleLogger::init(cli.verbose.log_level_filter(), Config::default());

    fn do_print_key<RS>(
        hive: &mut Hive<RS, CleanHive>,
        root_key: &KeyNode,
        cli: &Args,
    ) -> Result<()>
    where
        RS: Read + Seek,
    {
        let mut path = Vec::new();
        print_key(hive, root_key, &mut path, cli)
    }

    match File::open(&cli.hive_file) {
        Ok(data) => {
            let hive = Hive::new(data, cli.parse_mode()).unwrap();

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

            let root_key = &clean_hive.root_key_node().unwrap();
            do_print_key(&mut clean_hive, root_key, &cli).unwrap();
        }
        Err(why) => {
            eprintln!(
                "unable to open '{}': {}",
                cli.hive_file.to_string_lossy(),
                why
            );
            std::process::exit(-1);
        }
    }
    Ok(())
}

fn print_key<RS>(
    hive: &mut Hive<RS, CleanHive>,
    keynode: &KeyNode,
    path: &mut Vec<String>,
    cli: &Args,
) -> Result<()>
where
    RS: Read + Seek,
{
    path.push(keynode.name().to_string());

    let current_path = path.join("\\");
    if cli.display_bodyfile {
        let bf_line = Bodyfile3Line::new()
            .with_name(&current_path)
            .with_ctime(keynode.timestamp().timestamp());
        println!("{}", bf_line);
    } else {
        if cli.hide_timestamps {
            println!("\n[{}]", &current_path);
        } else {
            println!("\n[{}]; {}", &current_path, keynode.timestamp());
        }

        print_values(keynode);
    }

    for sk in keynode.subkeys(hive).unwrap().iter() {
        print_key(hive, &sk.borrow(), path, cli)?;
    }
    path.pop();

    Ok(())
}

fn print_values(keynode: &KeyNode) {
    for value in keynode.values() {
        let data_type = match value.data_type() {
            Some(dt) => format!("{dt}:"),
            None => "".into(),
        };

        println!("\"{}\" = {data_type}{}", value.name(), value.value());
    }
}
