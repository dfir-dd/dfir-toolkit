use dfir_toolkit::common::bodyfile::{BehavesLikeI64, Bodyfile3Line};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::io::{Stdout, Write};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::error::MactimeError;
use crate::filter::{Joinable, RunOptions, Runnable, Sorter};

use super::MACBFlags;

pub trait Mactime2Writer<W>: Send
where
    W: Write + Send
{
    fn write_line(&mut self, timestamp: &i64, entry: &ListEntry) -> std::io::Result<()>;
    fn into_writer(self) -> W;
}

#[derive(Default)]
pub struct BodyfileSorter {
    worker: Option<JoinHandle<Result<(), MactimeError>>>,
    receiver: Option<Receiver<Bodyfile3Line>>,
    output: Option<Box<dyn Mactime2Writer<Stdout>>>,
}

#[derive(Debug)]
pub struct ListEntry {
    pub flags: MACBFlags,
    pub line: Arc<Bodyfile3Line>,
}

impl Eq for ListEntry {}
impl PartialEq for ListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.line.get_inode().eq(other.line.get_inode())
            && self.line.get_name().eq(other.line.get_name())
    }
}
impl PartialOrd for ListEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.get_name().cmp(other.line.get_name()) {
            Ordering::Equal => self.line.get_inode().cmp(other.line.get_inode()),
            other => other,
        }
    }
}

fn insert_timestamp(
    entries: &mut BTreeMap<i64, Vec<ListEntry>>,
    flag: MACBFlags,
    line: Arc<Bodyfile3Line>,
) {
    let timestamp = if flag.contains(MACBFlags::M) {
        *line.get_mtime().as_ref().unwrap()
    } else if flag.contains(MACBFlags::A) {
        *line.get_atime().as_ref().unwrap()
    } else if flag.contains(MACBFlags::C) {
        *line.get_ctime().as_ref().unwrap()
    } else if flag.contains(MACBFlags::B) {
        *line.get_crtime().as_ref().unwrap()
    } else {
        -1
    };

    match entries.get_mut(&timestamp) {
        None => {
            let mut entries_at_ts = Vec::new();
            let entry = ListEntry { flags: flag, line };
            entries_at_ts.push(entry);
            entries.insert(timestamp, entries_at_ts);
        }

        Some(entries_at_ts) => {
            let entry = ListEntry { flags: flag, line };
            entries_at_ts.push(entry);
        }
    }
}

impl Runnable for BodyfileSorter {
    fn run(&mut self) {
        let receiver = self
            .receiver
            .take()
            .expect("no receiver provided; please call with_receiver()");
        let output = self
            .output
            .take()
            .expect("no output provided; please call with_output()");
        self.worker = Some(std::thread::spawn(move || Self::worker(receiver, output)));
    }
}

impl BodyfileSorter {
    pub fn with_receiver(mut self, decoder: Receiver<Bodyfile3Line>, _: RunOptions) -> Self {
        self.receiver = Some(decoder);
        self
    }

    pub fn with_output(mut self, output: Box<dyn Mactime2Writer<Stdout>>) -> Self {
        self.output = Some(output);
        self
    }

    fn worker(
        decoder: Receiver<Bodyfile3Line>,
        mut output: Box<dyn Mactime2Writer<Stdout>>,
    ) -> Result<(), MactimeError> {
        let mut entries: BTreeMap<i64, Vec<ListEntry>> = BTreeMap::new();
        let mut names: HashSet<(String, String)> = HashSet::new();

        loop {
            let line = Arc::new(match decoder.recv() {
                Err(_) => {
                    break;
                }
                Ok(l) => l,
            });

            // each name && inode SHOULD occur only once
            {
                let bf: &Bodyfile3Line = line.borrow();
                if names.contains(&(bf.get_inode().to_owned(), bf.get_name().to_owned())) {
                    log::warn!(
                        "ambigious file name: '{}' and inode '{}'",
                        bf.get_name(),
                        bf.get_inode()
                    );
                }
                names.insert((bf.get_inode().to_owned(), bf.get_name().to_owned()));
            } // delete the borrow to line

            // we need *some* value in mactimes!
            if line.get_mtime().is_none()
                && line.get_atime().is_none()
                && line.get_ctime().is_none()
                && line.get_crtime().is_none()
            {
                insert_timestamp(&mut entries, MACBFlags::NONE, Arc::clone(&line));
                continue;
            }

            let mut flags: [MACBFlags; 4] = [MACBFlags::NONE; 4];

            if line.get_mtime().is_some() {
                flags[0] |= MACBFlags::M;
            }
            if line.get_atime().is_some() {
                if line.get_mtime().as_ref() == line.get_atime().as_ref() {
                    flags[0] |= MACBFlags::A;
                } else {
                    flags[1] |= MACBFlags::A;
                }
            }
            if line.get_ctime().is_some() {
                if line.get_mtime().as_ref() == line.get_ctime().as_ref() {
                    flags[0] |= MACBFlags::C;
                } else if line.get_atime().as_ref() == line.get_ctime().as_ref() {
                    flags[1] |= MACBFlags::C;
                } else {
                    flags[2] |= MACBFlags::C;
                }
            }
            if line.get_crtime().is_some() {
                if line.get_mtime().as_ref() == line.get_crtime().as_ref() {
                    flags[0] |= MACBFlags::B;
                } else if line.get_atime().as_ref() == line.get_crtime().as_ref() {
                    flags[1] |= MACBFlags::B;
                } else if line.get_ctime().as_ref() == line.get_crtime().as_ref() {
                    flags[2] |= MACBFlags::B;
                } else {
                    flags[3] |= MACBFlags::B;
                }
            }
            for flag in flags.iter() {
                if flag != &MACBFlags::NONE {
                    insert_timestamp(&mut entries, *flag, Arc::clone(&line));
                }
            }
        }

        for (ts, entries_at_ts) in entries.iter() {
            for line in entries_at_ts {
                output.write_line(ts, line)?;
            }
        }
        Ok(())
    }
}

impl Joinable<Result<(), MactimeError>> for BodyfileSorter {
    fn join(&mut self) -> std::thread::Result<Result<(), MactimeError>> {
        self.worker.take().unwrap().join()
    }
}

impl Sorter<Result<(), MactimeError>> for BodyfileSorter {}
