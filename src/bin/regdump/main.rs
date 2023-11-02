use anyhow::{bail, Result};

use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use nt_hive2::*;
use simplelog::{Config, SimpleLogger};
use std::fs::File;
use std::io::{Read, Seek};

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let mut cli = Cli::parse_cli();
    let _ = SimpleLogger::init(cli.verbose.log_level_filter(), Config::default());

    fn do_print_key<RS>(hive: &mut Hive<RS, CleanHive>, root_key: &KeyNode, cli: &Cli) -> Result<()>
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
    cli: &Cli,
) -> Result<()>
where
    RS: Read + Seek,
{
    path.push(keynode.name().to_string());

    let current_path = path.join("\\");
    if cli.display_bodyfile {
        let bf_line = Bodyfile3Line::new()
            .with_name(&current_path)
            .with_ctime(keynode.timestamp().into());
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
