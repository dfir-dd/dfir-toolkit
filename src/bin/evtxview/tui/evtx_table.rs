use std::collections::HashSet;
use std::{fs::File, path::Path};

use chrono::{DateTime, Utc};
use dfir_toolkit::common::FormattableDatetime;
use evtx::{EvtxParser, SerializedEvtxRecord};
use ouroboros::self_referencing;
use quick_xml::de::from_str;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::HighlightSpacing;
use ratatui::{
    style::{Modifier, Style},
    text::Text,
    widgets::{Cell, Row, Table},
};

use crate::event::Event;

use super::color_scheme::{ColorScheme, PALETTES};

#[derive(Eq, PartialEq, Hash)]
pub enum EventFilter {
    ExcludeByEventId(u32),
    IncludeByEventId(u32),
}

impl EventFilter {
    pub fn filter(&self, rc: &RowContents) -> bool {
        match self {
            EventFilter::ExcludeByEventId(event_id) => rc.event.system().EventID() != event_id,
            EventFilter::IncludeByEventId(event_id) => rc.event.system().EventID() == event_id,
        }
    }
}

pub struct EvtxTable {
    rows: Vec<RowContents>,
    sparkline_data: Vec<u64>,
    colors: ColorScheme,
    timestamp_width: u16,
    event_filters: HashSet<EventFilter>,
    filtered_rows_count: usize,
}

impl TryFrom<&Path> for EvtxTable {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut rows: Vec<_> = RowContentsIterator::try_from(path)?.collect();
        rows.sort_by(|lhs, rhs| lhs.record_timestamp.cmp(&rhs.record_timestamp));
        let mut sparkline_data = Vec::new();
        if let Some(first_ts) = rows.first() {
            if let Some(last_ts) = rows.last() {
                let mut first_ts = first_ts.record_timestamp.timestamp();
                let last_ts = last_ts.record_timestamp.timestamp();
                assert!(last_ts >= first_ts);
                let step_size = i64::max(1, (last_ts - first_ts) / 3600);

                first_ts /= step_size;

                for row in rows.iter() {
                    let ts =
                        usize::try_from((row.record_timestamp.timestamp() / step_size) - first_ts)?;
                    while ts + 1 > sparkline_data.len() {
                        sparkline_data.push(0)
                    }
                    sparkline_data[ts] += 1;
                }
            }
        }
        let timestamp_width = u16::try_from(
            FormattableDatetime::from(chrono::offset::Utc::now())
                .to_string()
                .len(),
        )?;
        let filtered_rows_count = rows.len();
        Ok(EvtxTable {
            rows,
            colors: ColorScheme::new(&PALETTES[0]),
            timestamp_width,
            sparkline_data,
            event_filters: HashSet::new(),
            filtered_rows_count,
        })
    }
}

impl EvtxTable {
    pub fn table(&self) -> Table<'_> {
        let header_style = Style::default()
            .fg(self.colors.header_fg())
            .bg(self.colors.header_bg());

        let column_headers = ["", "Timestamp", "Record#", "Event#", "UserID", "Data"];
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

        let rows: Vec<_> = self.filtered_rows().collect();

        let table = Table::new(
            rows,
            vec![
                Constraint::Length(2),
                Constraint::Length(self.timestamp_width),
                Constraint::Length(column_headers[1].len() as u16),
                Constraint::Length(column_headers[1].len() as u16),
                Constraint::Length(20),
                Constraint::Min(1),
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            bar.into(),
            bar.into(),
        ]))
        //.bg(self.colors.buffer_bg())
        .highlight_spacing(HighlightSpacing::Always);
        table
    }

    fn filtered_rows(&self) -> impl Iterator<Item = &RowContents> {
        self.rows.iter().filter(|rc| self.filter_row(rc))
    }

    fn filter_row(&self, rc: &RowContents) -> bool {
        self.event_filters
            .iter()
            .map(|filter| filter.filter(rc))
            .fold(true, |a, b| a & b)
    }

    pub fn len(&self) -> usize {
        self.filtered_rows_count
    }

    pub fn is_empty(&self) -> bool {
        self.filtered_rows_count == 0
    }

    pub fn content(&self, filtered_row_id: usize) -> Option<&String> {
        self.filtered_rows().nth(filtered_row_id).map(|r| &r.raw_value)
    }

    pub fn sparkline_data(&self) -> &Vec<u64> {
        &self.sparkline_data
    }

    pub fn event_id_in_row(&self, filtered_row_id: usize) -> Option<u32> {
        self.filtered_rows()
            .nth(filtered_row_id)
            .map(|r| *r.event.system().EventID())
    }

    pub fn exclude_event_id(&mut self, filtered_row_id: usize) {
        if let Some(event_id) = self.event_id_in_row(filtered_row_id) {
            self.event_filters
                .insert(EventFilter::ExcludeByEventId(event_id));
        }
        self.filtered_rows_count = self.filtered_rows().count();
    }
    pub fn include_event_id(&mut self, filtered_row_id: usize) {
        if let Some(event_id) = self.event_id_in_row(filtered_row_id) {
            self.event_filters
                .insert(EventFilter::IncludeByEventId(event_id));
        }
        self.filtered_rows_count = self.filtered_rows().count();
    }
    pub fn reset_filter(&mut self) {
        self.event_filters.clear();
        self.filtered_rows_count = self.filtered_rows().count();
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
    record_timestamp: DateTime<Utc>,
    event_record_id: u64,
    level: String,
    timestamp: String,
    record_id: String,
    event_id: String,
    raw_value: String,
    user_id: String,
    event_data: String,
    event: Event,
}

impl<'r> TryFrom<&'r SerializedEvtxRecord<String>> for RowContents {
    type Error = anyhow::Error;

    fn try_from(record: &'r SerializedEvtxRecord<String>) -> Result<Self, Self::Error> {
        let event: Event = from_str(&record.data)?;
        let event_data = match event.event_data() {
            Some(data) => match data.data() {
                Some(data) => {
                    let value: Vec<_> = data
                        .iter()
                        .map(|d| {
                            format!(
                                "{}: {}",
                                d.name().as_ref().map(|s| &s[..]).unwrap_or(""),
                                d.value().as_ref().map(|s| &s[..]).unwrap_or_default()
                            )
                        })
                        .collect();
                    value.join(", ")
                }
                None => "".into(),
            },
            None => "".into(),
        };
        let mut user_id = event
            .system()
            .security()
            .user_id()
            .clone()
            .unwrap_or_default();

        if user_id.len() > 38 {
            if let Some(l) = user_id.split('-').last() {
                user_id = l.into();
            }
        }

        Ok(Self {
            event_record_id: record.event_record_id,
            record_timestamp: record.timestamp,
            level: event.system().level().to_string(),
            timestamp: FormattableDatetime::from(record.timestamp).to_string(),
            record_id: record.event_record_id.to_string(),
            event_id: event.system().EventID().to_string(),
            raw_value: record.data.clone(),
            user_id,
            event,
            event_data,
        })
    }
}

impl<'r> From<&'r RowContents> for Row<'r> {
    fn from(contents: &'r RowContents) -> Self {
        let mut row = Row::new(vec![
            &contents.level[..],
            &contents.timestamp[..],
            &contents.record_id[..],
            &contents.event_id[..],
            &contents.user_id[..],
            &contents.event_data[..],
        ]);

        if !contents.user_id.is_empty() && !contents.user_id.contains('-') {
            if contents.user_id == "500" {
                row = row.bold().red()
            } else {
                row = row.fg(Color::Red)
            }
        }

        row
    }
}
