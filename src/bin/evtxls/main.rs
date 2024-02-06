mod cli;
mod highlighted_string;
mod system_field;

use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use anyhow::Result;
use cli::{Cli, SortOrder};
use colored::{control::SHOULD_COLORIZE, Colorize};
use dfirtk_eventdata::EventId;
use evtx::{EvtxParser, ParserSettings, SerializedEvtxRecord};

use highlighted_string::HighlightedStringBuilder;
use serde_json::Value;

use dfir_toolkit::common::{FancyParser, FormattableDatetime};

use crate::system_field::FilterBySystemField;

struct EvtxLs {
    cli: Cli,
    hs_builder: HighlightedStringBuilder,
}

impl EvtxLs {
    fn new() -> Self {
        let cli = Cli::parse_cli();
        let hs_builder = HighlightedStringBuilder::new(cli.highlight.clone());

        Self { cli, hs_builder }
    }

    fn run(self) -> Result<()> {
        let mut records = Vec::new();

        for f_name in self.cli.evtx_files.iter() {
            let path = PathBuf::from(&f_name);

            let settings = ParserSettings::default().num_threads(0);
            let parser = EvtxParser::from_path(path)?.with_configuration(settings);

            records.extend(self.read_records(parser)?);
        }

        match self.cli.sort_order {
            SortOrder::Storage => assert!(records.is_empty()),
            SortOrder::RecordId => {
                records.sort_by(|a, b| a.event_record_id.cmp(&b.event_record_id))
            }
            SortOrder::Time => records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)),
        }

        if !records.is_empty() {
            for record in records.into_iter() {
                self.display_record(&record)?;
            }
        }

        Ok(())
    }

    fn read_records<T: Read + Seek>(&self, mut parser: EvtxParser<T>) -> Result<Vec<SerializedEvtxRecord<Value>>> {
        if self.cli.display_colors {
            SHOULD_COLORIZE.set_override(true);
        }

        let mut records = Vec::new();

        for result in parser.records_json_value() {
            match result {
                Err(_) => (),
                Ok(record) => {
                    if let Some(not_before) = self.cli.not_before.as_ref() {
                        if &record.timestamp < not_before {
                            continue;
                        }
                    }

                    if let Some(not_after) = self.cli.not_after.as_ref() {
                        if &record.timestamp > not_after {
                            continue;
                        }
                    }

                    if !self.cli.included_event_ids.is_empty() {
                        let event_id = EventId::try_from(&record)?.into();
                        if !self.cli.included_event_ids.contains(&event_id) {
                            continue;
                        }
                    }

                    if !self.cli.excluded_event_ids.is_empty() {
                        let event_id = EventId::try_from(&record)?.into();
                        if self.cli.excluded_event_ids.contains(&event_id) {
                            continue;
                        }
                    }

                    if matches!(self.cli.sort_order, SortOrder::Storage) {
                        self.display_record(&record)?
                    } else {
                        records.push(record);
                    }
                }
            }
        }

        Ok(records)
    }

    fn display_record(&self, record: &SerializedEvtxRecord<Value>) -> Result<()> {
        let system_fields = if self.cli.hide_base_fields {
            "".to_owned()
        } else {
            let system_fields = <SerializedEvtxRecord<Value> as FilterBySystemField>::filter_fields(
                record,
                self.cli.display_system_fields.as_ref()
            )?;

            let line_parts: Vec<String> = if self.cli.delimiter.is_none() {
                system_fields
                    .iter()
                    .map(|f| f.value_with_padding())
                    .collect()
            } else {
                system_fields.iter().map(|f| f.to_string()).collect()
            };
            if line_parts.is_empty() {
                "".to_owned()
            } else {
                format!(
                    "{}{}",
                    line_parts.join(&self.cli.delimiter.unwrap_or(' ').to_string()),
                    &self.cli.delimiter.unwrap_or(' ')
                )
            }
        };

        let event_data = self
            .format_custom_data(record, "UserData")
            .or_else(|| self.format_custom_data(record, "EventData"))
            .unwrap_or_else(|| "".to_owned())
            .replace("\\u001b", "\u{001b}");

        let timestamp = FormattableDatetime::from(&record.timestamp);
        let delimiter = self.cli.delimiter.unwrap_or(' ');

        let output=format!("{timestamp}{delimiter}{system_fields}{event_data}").normal();
        println!("{output}");

        Ok(())
    }

    fn format_custom_data(
        &self,
        record: &SerializedEvtxRecord<Value>,
        tag_name: &str,
    ) -> Option<String> {
        // fail if the event has no "Event" content
        let event = record.data.get("Event").unwrap();

        match event.get(tag_name) {
            None => None,
            Some(custom_data) => match custom_data {
                Value::Null => Some("".to_owned()),
                v => Some(self.hs_builder.highlight_data(v).to_string())
            },
        }
    }
}

fn main() -> Result<()> {
    sigpipe::reset();
    EvtxLs::new().run()
}
