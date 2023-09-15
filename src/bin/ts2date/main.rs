use std::io::{self, BufRead, Write};
use regex::Regex;
use dfir_toolkit::common::UnixTimestamp;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let re = Regex::new(r"^(?P<lhs>.*?)(?P<ts>\d{10})(?P<rhs>.*)$").unwrap();
    
    for line in stdin.lock().lines() {
        let content = line?;
        let out = match re.captures(&content) {
            Some(caps) => format!("{}{}{}", caps.name("lhs").unwrap().as_str(),
                                            UnixTimestamp::ts2date(caps.name("ts").unwrap().as_str().parse::<i64>().unwrap()),
                                            caps.name("rhs").unwrap().as_str()),
            None => content
        };
        
        io::stdout().write_all((out+ "\n").as_bytes())?;
    }
    Ok(())
}
