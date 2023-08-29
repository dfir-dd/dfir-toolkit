use clap::Parser;
use log::LevelFilter;
use crate::Protocol;
use dfir_toolkit::common::HasVerboseFlag;

#[cfg(feature = "gzip")]
const INPUTFILE_HELP: &str = "path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)";
#[cfg(not(feature = "gzip"))]
const INPUTFILE_HELP: &str = "path to input file or '-' for stdin";

#[derive(clap::Subcommand)]
pub (crate) enum Action {
    // create a new index
    CreateIndex,

    // import timeline data
    Import {
        #[clap(default_value="-", help=INPUTFILE_HELP)]
        input_file: String,

        /// number of timeline entries to combine in one bulk operation
        #[clap(long("bulk-size"), default_value_t=1000)]
        bulk_size: usize
    }
}

#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) action: Action,

    /// strict mode: do not only warn, but abort if an error occurs
    #[clap(num_args=0, long("strict"), display_order(500))]
    pub(crate) strict_mode: bool,

    /// name of the elasticsearch index
    #[clap(num_args=1, short('I'), long("index"), display_order = 800)]
    pub(crate) index_name: String,

    /// server name or IP address of elasticsearch server
    #[clap(num_args=1, 
        short('H'),
        long("host"),
        display_order = 810,
        default_value = "localhost"
    )]
    pub(crate) host: String,

    /// API port number of elasticsearch server
    #[clap(num_args=1, short('P'), long("port"), display_order = 820, default_value_t = 9200)]
    pub(crate) port: u16,

    /// protocol to be used to connect to elasticsearch
    #[clap(num_args=1, long("proto"), display_order=830, default_value_t=Protocol::Https)]
    pub(crate) protocol: Protocol,

    /// omit certificate validation
    #[clap(num_args=0, 
        short('k'),
        long("insecure"),
        display_order = 840,
        default_value_t = false
    )]
    pub(crate) omit_certificate_validation: bool,

    /// username for elasticsearch server
    #[clap(num_args=1, short('U'), long("username"), display_order=850, default_value=Some("elastic"))]
    pub(crate) username: String,

    /// password for authenticating at elasticsearch
    #[clap(num_args=1, short('W'), long("password"), display_order = 860)]
    pub(crate) password: String,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self)-> LevelFilter {
        self.verbose.log_level_filter()
    }
}