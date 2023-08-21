use anyhow::Result;
use colored::control::SHOULD_COLORIZE;
use ip_filter::IpFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    net::Ipv4Addr,
    path::PathBuf,
};

use clap::Parser;

mod format_ipv4;
mod ip_filter;
mod ipv4_with_properties;
use crate::format_ipv4::format_ipv4;

#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author,version,about,long_about=None)]
struct IPGrepApp {
    file: Vec<PathBuf>,

    /// display only lines who match ALL of the specified criteria. Values are delimited with comma
    #[clap(
        short('i'),
        long("include"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    include: Vec<IpFilter>,

    /// hide lines who match ANY of the specified criteria. Values are delimited with comma
    #[clap(
        short('x'),
        long("exclude"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    exclude: Vec<IpFilter>,

    /// ignore any of the specified IP addresses. Values are delimited with comma
    #[clap(
        short('I'),
        long("ignore-ips"),
        value_enum,
        use_value_delimiter = true,
        value_delimiter = ',',
        ignore_case = true
    )]
    ignore_ips: Vec<Ipv4Addr>,

    /// highlight interesting content using colors
    #[clap(short('c'), long("colors"))]
    display_colors: bool,

    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,

    /// print help in markdown format
    #[arg(long, hide = true, exclusive=true)]
    pub markdown_help: bool,
}
fn main() -> Result<()> {
    if std::env::args().any(|a| &a == "--markdown-help") {
        clap_markdown::print_help_markdown::<IPGrepApp>();
        return Ok(());
    }
    let app = IPGrepApp::parse();

    TermLogger::init(
        app.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    if app.display_colors {
        SHOULD_COLORIZE.set_override(true);
    }

    if app.file.is_empty() {
        app.ipgrep(std::io::stdin().lock())?;
    } else {
        for file in app.file.iter() {
            let f = File::open(file)?;
            app.ipgrep(BufReader::new(f))?;
        }
    }

    Ok(())
}

impl IPGrepApp {
    fn ipgrep<R: BufRead>(&self, mut reader: R) -> Result<()> {
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
