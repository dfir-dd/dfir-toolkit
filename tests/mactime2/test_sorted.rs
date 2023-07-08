use libmactime2::*;
use std::sync::mpsc::{self, Sender, Receiver};
use std::cell::RefCell;

#[macro_use]
extern crate more_asserts;

#[test]
fn test_sorted() {
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
    for day in 0..364 {
        for hour in 0..23 {
            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 1))
                .with_atime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 2))
                .with_mtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 3))
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 4))
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 5))
                .with_atime(random_ts())
                .with_mtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 6))
                .with_atime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 7))
                .with_atime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 8))
                .with_mtime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 9))
                .with_mtime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 10))
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 11))
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_ctime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 12))
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 13))
                .with_atime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 14))
                .with_mtime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();

            let bf = ::bodyfile::Bodyfile3Line::new()
                .with_name(&format!("sample_{}_{}_{}", day, hour, 15))
                .with_atime(random_ts())
                .with_mtime(random_ts())
                .with_ctime(random_ts())
                .with_crtime(random_ts());
            tx.send(bf.to_string()).unwrap();
        }
    }
    drop(tx);

    decoder.join().unwrap();
    let _ = sorter.join().unwrap();
}

fn random_ts() -> i64 {
    rand::random::<u32>() as i64
}

struct EventCatcher {
    last_timestamp: RefCell<i64>,
}

impl EventCatcher {
    pub fn new () -> Self {
        Self {
            last_timestamp: RefCell::new(-1)
        }
    }
}

impl Mactime2Writer for EventCatcher {
    fn fmt(&self, timestamp: &i64, _entry: &ListEntry) -> String {
        assert_le!(*self.last_timestamp.borrow(), *timestamp);

        *self.last_timestamp.borrow_mut() = *timestamp;
        "".to_owned()
    }
}