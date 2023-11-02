use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet},
    sync::{mpsc::Receiver, Arc},
    thread::JoinHandle,
};

use chrono_tz::Tz;
use dfir_toolkit::{
    common::bodyfile::Bodyfile3Line,
    es4forensics::{objects::PosixFile, Timestamp, TimelineObject},
};
use std::convert::TryFrom;

use crate::{
    error::MactimeError,
    filter::RunOptions,
    filter::{Consumer, Joinable, Runnable, Sorter},
};
pub struct JsonSorter {
    worker: Option<JoinHandle<Result<(), MactimeError>>>,
    receiver: Option<Receiver<Bodyfile3Line>>,
    src_zone: Tz,
}

impl Joinable<Result<(), MactimeError>> for JsonSorter {
    fn join(&mut self) -> std::thread::Result<Result<(), MactimeError>> {
        self.worker.take().unwrap().join()
    }
}

impl Consumer<Bodyfile3Line> for JsonSorter {
    fn with_receiver(previous: Receiver<Bodyfile3Line>, options: RunOptions) -> Self {
        Self {
            receiver: Some(previous),
            worker: None,
            src_zone: options.src_zone,
        }
    }
}

impl Runnable for JsonSorter {
    fn run(&mut self) {
        let receiver = self
            .receiver
            .take()
            .expect("no receiver provided; please call with_receiver()");
        let src_zone = self.src_zone;
        self.worker = Some(std::thread::spawn(move || {
            Self::json_worker(receiver, src_zone)
        }));
    }
}

impl Sorter<Result<(), MactimeError>> for JsonSorter {}

impl JsonSorter {
    fn json_worker(decoder: Receiver<Bodyfile3Line>, src_zone: Tz) -> Result<(), MactimeError> {
        let mut entries: BTreeMap<Timestamp, BTreeSet<String>> = BTreeMap::new();
        loop {
            let line = Arc::new(match decoder.recv() {
                Err(_) => {
                    break;
                }
                Ok(l) => l,
            });

            let bfline: &Bodyfile3Line = line.borrow();
            let pf = PosixFile::try_from((bfline, &src_zone)).unwrap();

            let lines: Vec<(Timestamp, String)> = pf
                .into_tuples()
                .map(|(t, v)| (t, serde_json::to_string(&v).unwrap()))
                .collect();

            if lines.is_empty() {
                log::warn!("file {} has no timestamp entries", line.get_name());
                log::warn!("raw entry is {}", line.to_string());
            } else {
                for (ts, line) in lines {
                    entries.entry(ts).or_default().insert(line);
                }
            }
        }

        for lines in entries.into_values() {
            for line in lines {
                println!("{}", line);
            }
        }
        Ok(())
    }
}
