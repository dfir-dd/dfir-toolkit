use std::{fs::File, path::Path};

use dfir_toolkit::common::FormattableDatetime;
use dfirtk_eventdata::EventId;
use evtx::{EvtxParser, SerializedEvtxRecord};
use ouroboros::self_referencing;
use ratatui::{
    style::{Modifier, Style},
    widgets::{Cell, Row, Table},
};
use serde_json::Value;

use super::color_scheme::{ColorScheme, PALETTES};

pub struct EvtxTable {
    rows: Vec<RowContents>,
    colors: ColorScheme,
    timestamp_width: u16,
}

impl TryFrom<&Path> for EvtxTable {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let rows = RowContentsIterator::try_from(path)?.collect();
        let timestamp_width = u16::try_from(
            FormattableDatetime::from(chrono::offset::Utc::now())
                .to_string()
                .len(),
        )?;
        Ok(EvtxTable {
            rows,
            colors: ColorScheme::new(&PALETTES[0]),
            timestamp_width,
        })
    }
}

impl EvtxTable {
    pub fn table(&self) -> Table<'_> {
        let header_style = Style::default()
            .fg(self.colors.header_fg())
            .bg(self.colors.header_bg());

        let header = ["Timestamp", "Record#", "Event#"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_style_fg());
        let table = Table::new(&self.rows, vec![self.timestamp_width, 10, 10])
            .header(header)
            .highlight_style(selected_style);
        table
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn content(&self, idx: usize) -> Option<&Value> {
        self.rows.get(idx).map(|r| &r.value)
    }
}

#[self_referencing]
pub struct RowContentsIterator {
    parser: EvtxParser<File>,

    #[borrows(mut parser)]
    #[covariant]
    iterator: Box<
        dyn Iterator<Item = evtx::err::Result<SerializedEvtxRecord<serde_json::Value>>> + 'this,
    >,
}

impl TryFrom<&Path> for RowContentsIterator {
    type Error = anyhow::Error;

    fn try_from(evtx_file: &Path) -> Result<Self, Self::Error> {
        let parser = EvtxParser::from_path(evtx_file)?;
        Ok(RowContentsIteratorBuilder {
            parser,
            iterator_builder: |parser| {
                Box::new(parser.serialized_records(|record| {
                    record.and_then(|record| record.into_json_value())
                }))
            },
        }
        .build())
    }
}

impl Iterator for RowContentsIterator {
    type Item = RowContents;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator_mut(|iterator| match iterator.next() {
            Some(Err(why)) => panic!("Error while reading record: {why}"),
            Some(Ok(r)) => Some((&r).into()),
            None => None,
        })
    }
}

pub struct RowContents {
    timestamp: String,
    record_id: String,
    event_id: String,
    value: Value,
}

impl<'r> From<&'r SerializedEvtxRecord<Value>> for RowContents {
    fn from(record: &'r SerializedEvtxRecord<Value>) -> Self {
        Self {
            timestamp: FormattableDatetime::from(record.timestamp).to_string(),
            record_id: record.event_record_id.to_string(),
            event_id: EventId::try_from(record)
                .ok()
                .map(|e| e.to_string())
                .unwrap_or_default(),
            value: record.data.clone(),
        }
    }
}

impl<'r> From<&'r RowContents> for Row<'r> {
    fn from(contents: &'r RowContents) -> Self {
        Row::new(vec![
            &contents.timestamp[..],
            &contents.record_id[..],
            &contents.event_id[..],
        ])
    }
}
