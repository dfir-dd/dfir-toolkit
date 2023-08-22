use std::{default, process::exit};

use clap::{value_parser, Arg, ArgAction, Parser};
use clap_complete::{Generator, Shell};

pub trait FancyParser<P: Parser> {
    fn parse_cli(bin_name: &str) -> P;

    fn parse_markdown_help();
    fn parse_autocomplete(bin_name: &str);
}

impl<P> FancyParser<P> for P
where
    P: Parser,
{
    fn parse_cli(bin_name: &str) -> P {
        Self::parse_markdown_help();
        Self::parse_autocomplete(bin_name);
        P::parse()
    }

    fn parse_markdown_help() {
        let matches = P::command()
            .ignore_errors(true)
            .arg(Arg::new("markdown-help").long("markdown-help"))
            .get_matches();

        if matches.contains_id("markdown-help") {
            clap_markdown::print_help_markdown::<P>();
            exit(0);
        }
    }

    fn parse_autocomplete(bin_name: &str) {
        let matches = P::command()
            .ignore_errors(true)
            .arg(
                Arg::new("autocomplete")
                    .long("autocomplete")
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(Shell)),
            )
            .get_matches();

        if let Some(generator) = matches.get_one::<Shell>("autocomplete") {
            generator.generate(&P::command().bin_name(bin_name), &mut std::io::stdout());
            exit(0);
        }
    }
}
