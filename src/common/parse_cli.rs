use std::process::exit;

use clap::{value_parser, Arg, ArgAction, Parser};
use clap_complete::{Generator, Shell};
use log::LevelFilter;
use simplelog::{SimpleLogger, Config};

pub trait HasVerboseFlag {
    fn log_level_filter(&self)-> LevelFilter;
}

pub trait FancyParser<P: Parser> {
    fn parse_cli() -> P;

    fn parse_markdown_help();
    fn parse_autocomplete();
}

impl<P> FancyParser<P> for P
where
    P: Parser + HasVerboseFlag,
{
    fn parse_cli() -> P {
        Self::parse_markdown_help();
        Self::parse_autocomplete();
        let cli = P::parse();

        let _ = SimpleLogger::init(cli.log_level_filter(), Config::default());
        cli
    }

    fn parse_markdown_help() {
        let matches = P::command()
            .ignore_errors(true)
            .arg(Arg::new("markdown-help").long("markdown-help").hide(true))
            .get_matches();

        if matches.contains_id("markdown-help") {
            clap_markdown::print_help_markdown::<P>();
            exit(0);
        }
    }

    fn parse_autocomplete() {
        let cmd = P::command();
        let matches = cmd.clone()
            .ignore_errors(true)
            .arg(
                Arg::new("autocomplete")
                    .long("autocomplete")
                    .action(ArgAction::Set)
                    .hide(true)
                    .value_parser(value_parser!(Shell)),
            )
            .get_matches();

        if let Some(generator) = matches.get_one::<Shell>("autocomplete") {
            let bin_name = cmd.get_name();
            let mut cmd = P::command().bin_name(bin_name);
            //let _ = cmd.get_subcommands_mut().map(|s|s.set_bin_name(bin_name));
            
            generator.generate(&P::command().bin_name(cmd.get_name()), &mut std::io::stdout());
            exit(0);
        }
    }
}
