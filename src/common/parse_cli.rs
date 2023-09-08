use std::process::exit;

use clap::{value_parser, Arg, ArgAction, Parser, Command};
use clap_complete::{generate, Generator, Shell};
use log::LevelFilter;
use simplelog::{Config, TermLogger, TerminalMode, ColorChoice};

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

        let _ = TermLogger::init(
            cli.log_level_filter(),
            Config::default(),
            TerminalMode::Stderr,
            ColorChoice::Auto);
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
            let mut cmd = P::command();
            
            print_completions(*generator, &mut cmd);
            exit(0);
        }
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}