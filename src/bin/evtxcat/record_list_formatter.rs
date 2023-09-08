use std::collections::HashMap;

use colored_json::to_colored_json_auto;
use evtx::SerializedEvtxRecord;
use term_table::{row::Row, table_cell::TableCell};

use crate::cli::Cli;

pub (crate) trait RecordListFormatter: Sized {
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