use clap::{Parser, ValueHint};
use clio::Input;
use log::LevelFilter;
use chrono_tz::Tz;

use dfir_toolkit::common::{HasVerboseFlag,TzArgument};

use super::OutputFormat;

#[cfg(feature = "gzip")]
const BODYFILE_HELP: &str =
    "path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)";
#[cfg(not(feature = "gzip"))]
const BODYFILE_HELP: &str = "path to input file or '-' for stdin";

const AFTER_HELP: &str = color_print::cstr!(r##"<red><bold>IMPORTANT</bold>

Note that POSIX specifies that all UNIX timestamps are UTC timestamps. It is
up to you to ensure that the bodyfile only contains UNIX timestamps that
comply with the POSIX standard.</red>"##);

/// Replacement for `mactime`
#[derive(Parser)]
#[clap(name="mactime2", author, version, long_about = None, after_help=AFTER_HELP)]

pub struct Cli {
    #[clap(short('b'), value_parser, value_hint=ValueHint::FilePath, default_value="-", help=BODYFILE_HELP, display_order(100))]
    pub(crate) input_file: Input,

    /// output format, if not specified, default value is 'txt'
    #[clap(
        short('F'),
        long("format"),
        value_enum,
        display_order(600)
    )]
    pub(crate) output_format: Option<OutputFormat>,

    /// output as CSV instead of TXT. This is a convenience option, which is identical to `--format=csv`
    /// and will be removed in a future release. If you specified `--format` and `-d`, the latter will be ignored.
    #[clap(short('d'), display_order(610))]
    #[arg(group="csv")]
    pub(crate) csv_format: bool,

    /// use the old CSV format that was used by legacy mactime.
    /// 
    /// Keep in mind that in this format, fields which contain commas will
    /// not be wrapped by quotes, as RFC4180 requires it. So, this format
    /// is not RFC4180-compliant, which means that you might not be able
    /// to use the output together with csv processing tools.
    #[clap(long("old-csv"), display_order(615))]
    pub(crate) old_csv: bool,

    /// output as JSON instead of TXT. This is a convenience option, which is identical to `--format=json`
    /// and will be removed in a future release. If you specified `--format` and `-j`, the latter will be ignored.
    #[clap(short('j'), display_order(620))]
    pub(crate) json_format: bool,

    /// name of offset of destination timezone (or 'list' to display all possible values
    #[clap(short('t'), long("to-timezone"), display_order(400), default_value_t=TzArgument::Tz(Tz::UTC))]
    pub dst_zone: TzArgument,

    // /// convert only, but do not sort
    // #[clap(short('c'), long("convert-only"), display_order(450))]
    // pub(crate) dont_sort: bool,
    /// strict mode: do not only warn, but abort if an error occurs
    #[clap(long("strict"), display_order(500))]
    pub(crate) strict_mode: bool,

    #[clap(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl HasVerboseFlag for Cli {
    fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}
