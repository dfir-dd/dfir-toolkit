use anyhow::{bail, Result};
use cli::Cli;
use dfir_toolkit::common::FancyParser;
use nt_hive2::{transactionlog::TransactionLog, ContainsHive, Hive, HiveParseMode};

mod cli;

pub fn main() -> Result<()> {
    let mut cli: Cli = Cli::parse_cli();

    let hive = Hive::new(&mut cli.hive_file, HiveParseMode::NormalWithBaseBlock).unwrap();

    let mut clean_hive = match cli.logfiles.len() {
        0 => {
            log::warn!("no log files provided, treating hive as if it was clean");
            hive.treat_hive_as_clean()
        }
        1 => match cli.logfiles.pop().unwrap().open()?.get_file() {
            Some(logfile1) => hive
                .with_transaction_log(TransactionLog::try_from(logfile1)?)?
                .apply_logs(),
            None => {
                bail!("logfile was not specified as file");
            }
        },
        2 => match cli.logfiles.pop().unwrap().open()?.get_file() {
            Some(logfile1) => match cli.logfiles.pop().unwrap().open()?.get_file() {
                Some(logfile2) => hive
                    .with_transaction_log(TransactionLog::try_from(logfile1)?)?
                    .with_transaction_log(TransactionLog::try_from(logfile2)?)?
                    .apply_logs(),
                None => {
                    bail!("logfile was not specified as file");
                }
            },
            None => {
                bail!("logfile was not specified as file");
            }
        },
        _ => {
            bail!("more than two transaction log files are not supported")
        }
    };

    clean_hive.write_baseblock(&mut cli.dst_hive)?;
    std::io::copy(&mut clean_hive, &mut cli.dst_hive)?;
    Ok(())
}
