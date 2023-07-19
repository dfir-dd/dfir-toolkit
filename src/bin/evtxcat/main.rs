use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use colored_json::to_colored_json_auto;
use evtx::{EvtxParser, SerializedEvtxRecord};

mod event_id;

mod range;
use term_table::{row::Row, table_cell::TableCell};

/// Display one or more events from an evtx file
#[derive(Parser)]
#[clap(author,version,name=env!("CARGO_BIN_NAME"))]
struct Cli {
    /// Name of the evtx file to read from
    evtx_file: String,

    /// filter: minimal event record identifier
    #[clap(long)]
    min: Option<u64>,

    /// filter: maximal event record identifier
    #[clap(long)]
    max: Option<u64>,

    /// show only the one event with this record identifier
    #[clap(short, long)]
    id: Option<u64>,

    /// don't display the records in a table format
    #[clap(short('T'), long("display-table"))]
    show_table: bool,

    #[clap(value_enum, short('F'), long("format"), default_value_t = OutputFormat::Xml)]
    format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Xml,
}

trait RecordFilter: Sized {
    type ReaderType: Read + Seek;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self>;

    fn filter_by_id(
        mut parser: EvtxParser<Self::ReaderType>,
        filter_id: u64,
    ) -> (Vec<u64>, HashMap<u64, SerializedEvtxRecord<Self>>) {
        let mut record_ids: Vec<u64> = Vec::new();
        let mut records: HashMap<u64, SerializedEvtxRecord<Self>> = HashMap::new();
        if let Some(result) = Self::unfiltered(&mut parser).find(|record| match record {
                    Ok(evt) => evt.event_record_id == filter_id,
                    _ => false,
                }) {
            let evt = result.unwrap();
            record_ids.push(evt.event_record_id);
            records.insert(evt.event_record_id, evt);
        }
        (record_ids, records)
    }

    fn filter_by_range(
        mut parser: EvtxParser<Self::ReaderType>,
        min: u64,
        max: u64,
    ) -> (Vec<u64>, HashMap<u64, SerializedEvtxRecord<Self>>) {
        let mut record_ids: Vec<u64> = Vec::new();
        let mut records: HashMap<u64, SerializedEvtxRecord<Self>> = HashMap::new();

        for record in Self::unfiltered(&mut parser) {
            match record {
                Err(_) => (),
                Ok(evt) => {
                    let id = evt.event_record_id;

                    if id >= min && id <= max {
                        record_ids.push(id);
                        records.insert(id, evt);
                    }
                }
            }
        }

        record_ids.sort_unstable();
        (record_ids, records)
    }
}

impl RecordFilter for serde_json::Value {
    type ReaderType = File;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self> {
        Unfiltered {
            inner: Box::new(parser.records_json_value()),
        }
    }
}

impl RecordFilter for String {
    type ReaderType = File;

    fn unfiltered(parser: &mut EvtxParser<Self::ReaderType>) -> Unfiltered<Self> {
        Unfiltered {
            inner: Box::new(parser.records()),
        }
    }
}

struct Unfiltered<'a, V> {
    inner: Box<dyn Iterator<Item = evtx::err::Result<SerializedEvtxRecord<V>>> + 'a>,
}

impl<'a, V> Iterator for Unfiltered<'a, V> {
    type Item = evtx::err::Result<SerializedEvtxRecord<V>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

trait RecordListFormatter: Sized {
    fn format(record: &SerializedEvtxRecord<Self>) -> String;

    fn display_results(
        record_ids: Vec<u64>,
        records: HashMap<u64, SerializedEvtxRecord<Self>>,
        cli: &Cli,
    ) {
        if !cli.show_table {
            for id in record_ids.into_iter() {
                let record = &records[&id];
                println!("{}", Self::format(record));
            }
        } else {
            let mut table = term_table::Table::new();
            if let Some(size) = termsize::get() {
                table.set_max_column_widths(vec![(0, 12), (1, (size.cols - 16).into())])
            }

            for id in record_ids.into_iter() {
                let record = &records[&id];
                table.add_row(Row::new(vec![
                    TableCell::new(id),
                    TableCell::new(Self::format(record)),
                ]));
            }
            println!("{}", table.render());
        }
    }
}

impl RecordListFormatter for String {
    fn format(record: &SerializedEvtxRecord<Self>) -> String {
        record.data.clone()
    }
}

impl RecordListFormatter for serde_json::Value {
    fn format(record: &SerializedEvtxRecord<Self>) -> String {
        to_colored_json_auto(&record.data).unwrap()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = PathBuf::try_from(&cli.evtx_file)?;

    let parser = EvtxParser::from_path(path)?;

    match cli.format {
        OutputFormat::Json => {
            let (record_ids, records) = if let Some(filter_id) = cli.id {
                serde_json::Value::filter_by_id(parser, filter_id)
            } else {
                let min = cli.min.unwrap_or(u64::MIN);
                let max = cli.max.unwrap_or(u64::MAX);
                serde_json::Value::filter_by_range(parser, min, max)
            };
            serde_json::Value::display_results(record_ids, records, &cli);
        }
        OutputFormat::Xml => {
            let (record_ids, records) = if let Some(filter_id) = cli.id {
                String::filter_by_id(parser, filter_id)
            } else {
                let min = cli.min.unwrap_or(u64::MIN);
                let max = cli.max.unwrap_or(u64::MAX);
                String::filter_by_range(parser, min, max)
            };
            String::display_results(record_ids, records, &cli);
        }
    }
    Ok(())
}
