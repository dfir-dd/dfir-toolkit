use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use chrono::Duration;
use clap::Parser;
use colored_json::to_colored_json_auto;
use evtx::{EvtxParser, SerializedEvtxRecord};
use libevtx::{EventId, Range};
use term_table::{row::Row, table_cell::TableCell};

/// Find time skews in an evtx file
#[derive(Parser)]
#[clap(author,version,name=env!("CARGO_BIN_NAME"))]
struct Cli {
    /// name of the evtx file to scan
    evtx_file: String,

    /// display also the contents of the records befor and after a time skew
    #[clap(short = 'S', long)]
    show_records: bool,

    /// negative tolerance limit (in seconds): time skews to the past below this limit will be ignored
    #[clap(short = 'N', long, default_value_t = 5)]
    negative_tolerance: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut record_ids: Vec<EventId> = Vec::new();
    let mut records: HashMap<EventId, SerializedEvtxRecord<serde_json::Value>> = HashMap::new();

    let path = PathBuf::try_from(&cli.evtx_file)?;

    let mut parser = EvtxParser::from_path(path)?;
    for record in parser.records_json_value() {
        match record {
            Err(_) => (),
            Ok(evt) => {
                let id = EventId::from(&evt);
                record_ids.push(id.clone());
                records.insert(id, evt);
            }
        }
    }
    record_ids.sort();

    let mut current_range = None;
    let mut ranges: Vec<Range> = Vec::new();

    for id in record_ids.iter() {
        if current_range.is_none() {
            current_range = Some(Range::from(id.clone()));
        } else {
            let range = current_range.as_mut().unwrap();
            if range.can_contain(id) {
                range.add_event(id.clone());
            } else {
                ranges.push(current_range.replace(Range::from(id.clone())).unwrap());
            }
        }
    }

    if let Some(range) = current_range.take() {
        ranges.push(range)
    }
    ranges.sort();
    print_ranges(&ranges, &records, &cli);
    Ok(())
}

fn print_ranges(
    ranges: &[Range],
    records: &HashMap<EventId, SerializedEvtxRecord<serde_json::Value>>,
    cli: &Cli,
) {
    let allowed_bias = Duration::seconds(cli.negative_tolerance.into());
    if cli.show_records {
        for range in ranges.iter() {
            let mut table = term_table::Table::new();
            if let Some(size) = termsize::get() {
                table.set_max_column_widths(vec![
                    (0, (size.cols / 2).into()),
                    (1, (size.cols / 2).into()),
                ])
            }

            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(
                    range.begin().timestamp().format("%FT%T"),
                    1,
                    term_table::table_cell::Alignment::Center,
                ),
                TableCell::new_with_alignment(
                    range.end().timestamp().format("%FT%T"),
                    1,
                    term_table::table_cell::Alignment::Center,
                ),
            ]));

            let first_record = &records[range.begin()];
            let last_record = &records[range.end()];
            table.add_row(Row::new(vec![
                TableCell::new(to_colored_json_auto(&first_record.data).unwrap()),
                TableCell::new(to_colored_json_auto(&last_record.data).unwrap()),
            ]));

            println!("{}", table.render());

            let mut table = term_table::Table::new();
            if let Some(size) = termsize::get() {
                table.set_max_column_widths(vec![
                    (0, 12),
                    (1, (size.cols / 2 - 8).into()),
                    (2, (size.cols / 2 - 8).into()),
                ])
            }
            let mut last_event: Option<&EventId> = None;
            for current_event in range.events() {
                if let Some(event) = last_event {
                    if *current_event.timestamp() + allowed_bias < *event.timestamp() {
                        table.add_row(Row::new(vec![
                            TableCell::new("time skew:"),
                            TableCell::new(format!(
                                "last event {} occurred at {}",
                                event.event_record_id(),
                                event.timestamp().format("%FT%T")
                            )),
                            TableCell::new(format!(
                                "current event {} occurred at {}",
                                current_event.event_record_id(),
                                current_event.timestamp().format("%FT%T")
                            )),
                        ]));

                        let record1 = &records[event];
                        let record2 = &records[current_event];
                        table.add_row(Row::new(vec![
                            TableCell::new(""),
                            TableCell::new(to_colored_json_auto(&record1.data).unwrap()),
                            TableCell::new(to_colored_json_auto(&record2.data).unwrap()),
                        ]));
                    }
                }
                last_event = Some(current_event);
            }
            println!("{}", table.render());
        }
    } else {
        for range in ranges.iter() {
            println!("RANGE: {}", range);
            println!("  {} events", range.len());

            let mut last_event: Option<&EventId> = None;
            for current_event in range.events() {
                if let Some(event) = last_event {
                    if *current_event.timestamp() + allowed_bias < *event.timestamp() {
                        println!("  time skew detected:");
                        println!(
                            "    last event            {} occurred at {},",
                            event.event_record_id(),
                            event.timestamp().format("%FT%T")
                        );
                        println!(
                            "    but the current event {} occurred at {}",
                            current_event.event_record_id(),
                            current_event.timestamp().format("%FT%T")
                        );

                        let duration = *current_event.timestamp() - *event.timestamp();
                        println!("    this is a duration of {}", duration);
                        println!();
                    }
                }
                last_event = Some(current_event);
            }
        }
    }
}
