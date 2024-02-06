use std::path::PathBuf;

use anyhow::Result;
use evtx::EvtxParser;

use dfir_toolkit::common::FancyParser;

mod cli;
mod output_format;
mod record_filter;
mod record_list_formatter;
mod unfiltered;

use cli::Cli;
use output_format::OutputFormat;
use record_filter::RecordFilter;
use record_list_formatter::RecordListFormatter;

fn main() -> Result<()> {
    let cli = Cli::parse_cli();

    let path = PathBuf::from(&cli.evtx_file);

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
