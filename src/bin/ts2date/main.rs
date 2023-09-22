use std::io::{BufRead, Write};
use regex::Regex;
use anyhow::{bail, Result};
use dfir_toolkit::common::{ForensicsTimestamp,FancyParser,TzArgument};
use cli::Cli;

mod cli;


fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    let mut input = cli.input_file;
    let mut output = cli.output_file;

    if cli.src_zone.is_list() || cli.dst_zone.is_list() {
        TzArgument::display_zones();
        return Ok(());
    }

    let re = Regex::new(r"^(?P<lhs>.*?)(?P<ts>\d{10})(?P<rhs>.*)$").unwrap();
    
    for line in input.lock().lines() {
        let content = match line {
            Ok(line) => line,
            Err(_) => bail!("content of input file need to be in UTF-8 (not in UTF-16)"),
        };

        let out = match re.captures(&content) {
            Some(caps) => {
                //let ndt = NaiveDateTime::from_timestamp_opt(caps.name("ts").unwrap().as_str().parse::<i64>().unwrap(),0).unwrap();
                let ts = ForensicsTimestamp::new(caps.name("ts").unwrap().as_str().parse::<i64>().unwrap(),cli.src_zone.into_tz().unwrap(), cli.dst_zone.into_tz().unwrap());
                format!("{}{}{}", caps.name("lhs").unwrap().as_str(),
                                            ts.format_date(),
                                            caps.name("rhs").unwrap().as_str())

            } 
            None => content
        };

        output.lock().write_all((out+ "\n").as_bytes())?;
        
    }
    Ok(())
}