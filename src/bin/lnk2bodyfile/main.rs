use dfir_toolkit::common::FancyParser;
use cli::Cli;
use anyhow::Result;
use lnk_parser;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    for filename in cli.lnk_files {
        match LnkFile::try_from(&filename[..]) {
            Ok(lnk_file) => lnk_file.print_bodyfile(),
            Err(why) => log::error!("unable to open {filename}: {why}"),
        }
    }

    println!("test");

    Ok(())
}