use ::bodyfile::Bodyfile3Line;
use libmactime2::*;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_ambiguity1() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let options = RunOptions {
        strict_mode: false,
        src_zone: chrono_tz::Tz::UTC
    };

    let mut decoder = BodyfileDecoder::with_receiver(rx, options);
    let mut sorter = BodyfileSorter::default()
        .with_receiver(decoder.get_receiver(), options)
        .with_output(Box::new(EventCatcher::new()));

    sorter.run();

    let ts1 = random_ts();
    let ts2 = ts1 + 1;
    let bf = ::bodyfile::Bodyfile3Line::new()
        .with_name("sample1.txt")
        .with_atime(ts1)
        .with_mtime(ts1)
        .with_ctime(ts1)
        .with_crtime(ts1);
    tx.send(bf.to_string()).unwrap();

    let bf = ::bodyfile::Bodyfile3Line::new()
        .with_name("sample1.txt")
        .with_atime(ts2)
        .with_mtime(ts2)
        .with_ctime(ts2)
        .with_crtime(ts2);
    tx.send(bf.to_string()).unwrap();

    drop(tx);

    decoder.join().unwrap();
    assert!(sorter.join().is_ok());
}

fn random_ts() -> i64 {
    rand::random::<u32>() as i64
}

struct EventCatcher {
    last_timestamp: RefCell<i64>,
    names: RefCell<HashSet<String>>,
}

impl EventCatcher {
    pub fn new () -> Self {
        Self {
            last_timestamp: RefCell::new(-1),
            names: RefCell::new(HashSet::new())
        }
    }
}

impl Mactime2Writer for EventCatcher {
    fn fmt(&self, timestamp: &i64, entry: &ListEntry) -> String {
        assert_le!(*self.last_timestamp.borrow(), *timestamp);
        *self.last_timestamp.borrow_mut() = *timestamp;

        let bf: &Bodyfile3Line = entry.line.borrow();
        //assert!(! self.names.borrow().contains(bf.get_name()));
        self.names.borrow_mut().insert(bf.get_name().to_owned());

        "".to_owned()
    }
}