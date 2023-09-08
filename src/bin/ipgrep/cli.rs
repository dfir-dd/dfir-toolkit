use std::{net::Ipv4Addr, path::PathBuf, io::BufRead};

use clap::Parser;
use dfir_toolkit::common::HasVerboseFlag;
use log::LevelFilter;

use crate::{ip_filter::IpFilter, format_ipv4};

#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author,version,about,long_about=None)]
pub(crate) struct Cli {
    pub(crate) file: Vec<PathBuf>,

    /// display only lines who match ALL of the specified criteria. Values are delimited with comma
    #[clap(
        short('i'),
        long("include"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    pub(crate) include: Vec<IpFilter>,

    /// hide lines who match ANY of the specified criteria. Values are delimited with comma
    #[clap(
        short('x'),
        long("exclude"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    pub(crate) exclude: Vec<IpFilter>,

    /// ignore any of the specified IP addresses. Values are delimited with comma
    #[clap(
        short('I'),
        long("ignore-ips"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    pub(crate) ignore_ips: Vec<Ipv4Addr>,

    /// highlight interesting content using colors
    #[clap(short('c'), long("colors"))]
    pub(crate) display_colors: bool,

    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}

impl Cli {
    pub (crate) fn ipgrep<R: BufRead>(&self, mut reader: R) -> anyhow::Result<()> {
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Err(_) => break,
                Ok(_) => {
                    if let Some(hline) = format_ipv4(
                        &self.exclude[..],
                        &self.include[..],
                        &self.ignore_ips[..],
                        &line,
                    ) {
                        print!("{hline}");
                    }
                }
            }
        }
        Ok(())
    }
}