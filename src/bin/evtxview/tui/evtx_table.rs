use std::{fs::File, path::Path};

use dfir_toolkit::common::FormattableDatetime;
use evtx::{EvtxParser, SerializedEvtxRecord};
use ouroboros::self_referencing;
use quick_xml::de::from_str;
use ratatui::style::Stylize;
use ratatui::widgets::HighlightSpacing;
use ratatui::{
    style::{Modifier, Style},
    text::Text,
    widgets::{Cell, Row, Table},
};

use crate::event::Event;

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

        let column_headers = ["", "Timestamp", "Record#", "Event#"];
        let header = column_headers
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_style_fg());

        let bar = " █ ";

        let table = Table::new(
            &self.rows,
            vec![
                1,
                self.timestamp_width,
                column_headers[1].len() as u16,
                column_headers[1].len() as u16,
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec!["".into(), bar.into(), bar.into()]))
        .bg(self.colors.buffer_bg())
        .highlight_spacing(HighlightSpacing::Always);
        table
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn content(&self, idx: usize) -> Option<&String> {
        self.rows.get(idx).map(|r| &r.raw_value)
    }
}

#[self_referencing]
pub struct RowContentsIterator {
    parser: EvtxParser<File>,

    #[borrows(mut parser)]
    #[covariant]
    iterator: Box<dyn Iterator<Item = evtx::err::Result<SerializedEvtxRecord<String>>> + 'this>,
}

impl TryFrom<&Path> for RowContentsIterator {
    type Error = anyhow::Error;

    fn try_from(evtx_file: &Path) -> Result<Self, Self::Error> {
        let parser = EvtxParser::from_path(evtx_file)?;
        Ok(RowContentsIteratorBuilder {
            parser,
            iterator_builder: |parser| Box::new(parser.records()),
        }
        .build())
    }
}

impl Iterator for RowContentsIterator {
    type Item = RowContents;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator_mut(|iterator| match iterator.next() {
            Some(Err(why)) => panic!("Error while reading record: {why}"),
            Some(Ok(r)) => match (&r).try_into() {
                Ok(contents) => Some(contents),
                Err(why) => panic!("Error while parsing record: {why}"),
            },
            None => None,
        })
    }
}

#[allow(dead_code)]
pub struct RowContents {
    level: String,
    timestamp: String,
    record_id: String,
    event_id: String,
    raw_value: String,
    event: Event,
}

impl<'r> TryFrom<&'r SerializedEvtxRecord<String>> for RowContents {
    type Error = anyhow::Error;

    fn try_from(record: &'r SerializedEvtxRecord<String>) -> Result<Self, Self::Error> {
        let event: Event = from_str(&record.data)?;
        Ok(Self {
            level: event.system().level().to_string(),
            timestamp: FormattableDatetime::from(record.timestamp).to_string(),
            record_id: record.event_record_id.to_string(),
            event_id: event.system().EventID().clone(),
            raw_value: record.data.clone(),
            event,
        })
    }
}

impl<'r> From<&'r RowContents> for Row<'r> {
    fn from(contents: &'r RowContents) -> Self {
        Row::new(vec![
            &contents.level[..],
            &contents.timestamp[..],
            &contents.record_id[..],
            &contents.event_id[..],
        ])
    }
}
