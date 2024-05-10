use chrono::{DateTime, Utc};
use dfir_toolkit::common::FormattableDatetime;
use evtx::SerializedEvtxRecord;
use getset::Getters;
use ratatui::{
    style::{Color, Stylize},
    widgets::Row,
};

use crate::event::Event;

#[allow(dead_code)]
#[derive(Getters)]
#[getset(get="pub")]
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
    channel: String,
    event: Event,
}

impl<'r> TryFrom<&'r SerializedEvtxRecord<String>> for RowContents {
    type Error = anyhow::Error;

    fn try_from(record: &'r SerializedEvtxRecord<String>) -> Result<Self, Self::Error> {
        let event: Event = quick_xml::de::from_str(&record.data)?;
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

        let channel = event.system().channel().clone().replace("Microsoft-Windows-", "");

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
            channel
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
            &contents.channel[..],
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

impl Ord for RowContents {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.record_timestamp.cmp(&other.record_timestamp) {
            std::cmp::Ordering::Equal => self.event_record_id.cmp(&other.event_record_id),
            res => res,
        }
    }
}

impl PartialOrd for RowContents {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for RowContents {}

impl PartialEq for RowContents {
    fn eq(&self, other: &Self) -> bool {
        self.record_timestamp.eq(&other.record_timestamp) && self.event_record_id.eq(&other.event_record_id)
    }
}
