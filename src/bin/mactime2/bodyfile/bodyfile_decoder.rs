use crate::apps::mactime2::filter::{Consumer, Filter, Joinable, Provider, RunOptions};
use crate::common::bodyfile::Bodyfile3Line;
use std::convert::TryFrom;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

pub struct BodyfileDecoder {
    worker: Option<JoinHandle<()>>,
    rx: Option<Receiver<Bodyfile3Line>>,
}

impl Filter<String, Bodyfile3Line, ()> for BodyfileDecoder {
    fn worker(reader: Receiver<String>, tx: Sender<Bodyfile3Line>, options: RunOptions) {
        loop {
            let mut line = match reader.recv() {
                Err(_) => {
                    break;
                }
                Ok(l) => l,
            };

            if line.starts_with('#') {
                continue;
            }
            Self::trim_newline(&mut line);

            let bf_line = match Bodyfile3Line::try_from(line.as_ref()) {
                Err(e) => {
                    if options.strict_mode {
                        log::warn!("bodyfile parser error: {}", e);
                        panic!("failed while parsing: {:?}", line);
                    } else {
                        log::warn!("bodyfile parser error: {}", e);
                        #[cfg(debug_assertions)]
                        log::warn!("failed line was: {:?}", line);
                    }
                    continue;
                }
                Ok(l) => l,
            };

            if tx.send(bf_line).is_err() {
                break;
            }
        }
    }
}

impl Provider<Bodyfile3Line, ()> for BodyfileDecoder {
    fn get_receiver(&mut self) -> Receiver<Bodyfile3Line> {
        self.rx.take().unwrap()
    }
}

impl Consumer<String> for BodyfileDecoder {
    fn with_receiver(reader: Receiver<String>, options: RunOptions) -> Self {
        let (tx, rx): (Sender<Bodyfile3Line>, Receiver<Bodyfile3Line>) = mpsc::channel();
        Self {
            worker: Some(std::thread::spawn(move || {
                Self::worker(reader, tx, options)
            })),
            rx: Some(rx),
        }
    }
}

impl BodyfileDecoder {
    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

impl Joinable<()> for BodyfileDecoder {
    fn join(&mut self) -> std::thread::Result<()> {
        self.worker.take().unwrap().join()
    }
}
