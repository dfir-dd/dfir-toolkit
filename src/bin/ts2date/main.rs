use std::io::{BufRead, Write};
use chrono::{DateTime, Utc};
use regex::Regex;
use anyhow::Result;
use dfir_toolkit::common::UnixTimestamp;
use dfir_toolkit::common::FancyParser;
use cli::Cli;

mod cli;


fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    let mut input = cli.input_file;
    let mut output = cli.output_file;

    let re = Regex::new(r"^(?P<lhs>.*?)(?P<ts>\d{10})(?P<rhs>.*)$").unwrap();
    
    for line in input.lock().lines() {
        let content = line?;
        let out = match re.captures(&content) {
            Some(caps) => format!("{}{}{}", caps.name("lhs").unwrap().as_str(),
                                            DateTime::<Utc>::ts2date(caps.name("ts").unwrap().as_str().parse::<i64>().unwrap()),
                                            caps.name("rhs").unwrap().as_str()),
            None => content
        };

        output.lock().write_all((out+ "\n").as_bytes())?;
        
    }
    Ok(())
}