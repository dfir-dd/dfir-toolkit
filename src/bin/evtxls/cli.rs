use clap::{Parser, ValueEnum};

use super::{Rfc3339Datetime, SystemField};
use regex::Regex;

#[derive(ValueEnum, Clone)]
pub(crate) enum SortOrder {
    /// don't change order, output records as they are stored
    Storage,

    /// sort by event record id
    RecordId,

    /// sort by date and time
    Time,
}

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
pub(crate) struct Cli {
    /// Name of the evtx files to read from
    pub(crate) evtx_files: Vec<String>,

    /// use this delimiter instead of generating fixed space columns
    #[clap(short('d'), long("delimiter"))]
    pub(crate) delimiter: Option<char>,

    /// List events with only the specified event ids, separated by ','
    #[clap(
        short('i'),
        long("include"),
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub(crate) included_event_ids: Vec<u16>,


    /// Exclude events with the specified event ids, separated by ','
    #[clap(
        short('x'),
        long("exclude"),
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub(crate) excluded_event_ids: Vec<u16>,


    /// highlight interesting content using colors
    #[clap(short('c'), long("colors"))]
    pub(crate) display_colors: bool,

    /// hide events older than the specified date (hint: use RFC 3339 syntax)
    #[clap(short('f'), long("from"))]
    pub(crate) not_before: Option<Rfc3339Datetime>,

    /// hide events newer than the specified date (hint: use RFC 3339 syntax)
    #[clap(short('t'), long("to"))]
    pub(crate) not_after: Option<Rfc3339Datetime>,

    /// highlight event data based on this regular expression
    #[clap(short('r'), long("regex"))]
    pub(crate) highlight: Option<Regex>,

    /// sort order
    #[clap(short('s'), long("sort"), value_enum, default_value_t=SortOrder::Storage)]
    pub(crate) sort_order: SortOrder,

    /// display fields common to all events. multiple values must be separated by ','
    #[clap(
        short('b'),
        long("base-fields"),
        value_enum,
        use_value_delimiter=true,
        value_delimiter=',',
        ignore_case=true,
        default_values_t=vec![SystemField::EventId, SystemField::EventRecordId])]
    pub(crate) display_system_fields: Vec<SystemField>,

    /// don't display any common event fields at all. This corresponds to
    /// specifying '--base-fields' without any values (which is not allowed, that's why there is this flag)
    #[clap(short('B'), long("hide-base-fields"), default_value_t=false)]
    pub (crate) hide_base_fields: bool,
}


