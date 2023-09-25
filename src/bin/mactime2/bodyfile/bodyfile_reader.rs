use encoding_rs_io::DecodeReaderBytesBuilder;
use std::io::{BufRead, BufReader, Read};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use crate::apps::mactime2::filter::{Joinable, Provider};
use crate::apps::mactime2::stream::{StreamReader, StreamWorker};

pub struct BodyfileReader {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<String>>,
}

impl Provider<String, ()> for BodyfileReader {
    fn get_receiver(&mut self) -> Receiver<String> {
        self.rx.take().unwrap()
    }
}

impl StreamWorker<String> for BodyfileReader {
    fn worker<R: Read + Send>(input: R, tx: Sender<String>) {
        let mut line_ctr = 1;

        let drb = DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::UTF_8))
            .utf8_passthru(true)
            .build(input);
        let mut reader = BufReader::new(drb);

        loop {
            let mut line = String::new();
            let size = reader.read_line(&mut line);

            match size {
                Err(why) => {
                    eprintln!("IO Error in line {}: {:?}", line_ctr, why);
                    break;
                }
                Ok(s) => {
                    if s == 0 {
                        break;
                    }

                    if tx.send(line).is_err() {
                        break;
                    }
                }
            }
            line_ctr += 1;
        }
    }
}

impl StreamReader<String, ()> for BodyfileReader {
    fn new(worker: JoinHandle<()>, rx: Receiver<String>) -> Self {
        Self {
            worker: Some(worker),
            rx: Some(rx),
        }
    }
}

impl Joinable<()> for BodyfileReader {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}
