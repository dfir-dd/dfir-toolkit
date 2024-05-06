use std::{
    mem,
    path::Path,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use cursive::{Cursive, View};
use cursive_async_view::{AsyncProgressState, AsyncProgressView};
use cursive_table_view::TableView;
use num_traits::cast::AsPrimitive;

use crate::{evtx_column::EvtxColumn, evtx_line::EvtxLine};

pub struct EvtxView {
    records_table: AsyncProgressView<TableView<EvtxLine, EvtxColumn>>,
}

impl EvtxView {
    pub fn new(siv: &mut Cursive, evtx_file: &Path) -> anyhow::Result<Self> {
        let number_of_records = Arc::new(Mutex::new(usize::MAX));
        let records = Arc::new(Mutex::new(Vec::new()));

        let reader = Self::start_reader_thread(
            evtx_file,
            Arc::clone(&number_of_records),
            Arc::clone(&records),
        );

        let async_view = AsyncProgressView::new(siv, move || {
            if reader.is_finished() {
                // take the vector stored in `records` and replace it with an empty record.
                // the result will be stored in `my_records`. This approach takes care
                // that no value is moved out of `reader`, which would be not allowed
                let mut my_records = Vec::new();
                if let Ok(mut records) = records.lock() {
                    my_records = mem::replace(&mut *records, my_records);
                }

                let records_table = TableView::<EvtxLine, EvtxColumn>::new()
                    .column(EvtxColumn::Timestamp, "Time", |c| c)
                    .column(EvtxColumn::EventRecordId, "Record#", |c| c)
                    .column(EvtxColumn::EventId, "Event#", |c| c)
                    .items(my_records);
                AsyncProgressState::Available(records_table)
            } else {
                match records.lock() {
                    Ok(records) => match number_of_records.lock() {
                        Ok(number_of_records) => {
                            if *number_of_records == usize::MAX {
                                AsyncProgressState::Pending(0.0)
                            } else {
                                let r_len: f32 = records.len().as_();
                                let number_of_records: f32 = number_of_records.as_();
                                AsyncProgressState::Pending(r_len / number_of_records)
                            }
                        }
                        Err(why) => AsyncProgressState::Error(format!("{why}")),
                    },
                    Err(why) => AsyncProgressState::Error(format!("{why}")),
                }
            }
        });

        Ok(Self {
            records_table: async_view,
        })
    }

    fn start_reader_thread(
        evtx_file: &Path,
        number_of_records: Arc<Mutex<usize>>,
        records: Arc<Mutex<Vec<EvtxLine>>>,
    ) -> JoinHandle<()> {
        let evtx_file = evtx_file.to_path_buf();
        thread::spawn(move || {
            if let Ok(mut number_of_records) = number_of_records.lock() {
                *number_of_records = evtx::EvtxParser::from_path(&evtx_file)
                    .unwrap()
                    .records()
                    .filter(Result::is_ok)
                    .count();
            }

            let mut parser = evtx::EvtxParser::from_path(evtx_file).unwrap();
            for res in parser.records_json_value().filter_map(Result::ok) {
                if let Ok(mut records) = records.lock() {
                    records.push(EvtxLine::from(res));
                }
            }
        })
    }
}

impl View for EvtxView {
    fn draw(&self, printer: &cursive::Printer) {
        self.records_table.draw(printer)
    }
}
