use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use anyhow::Result;
use clio::Input;

use crate::apps::mactime2::{stream::*, filter::{Joinable, Provider}};

pub(crate) trait StreamReader<T, R>: Sized + StreamWorker<T> + Joinable<R> + Provider<T, R>
where
    T: Send + 'static,
{
    fn from(input: Input) -> Result<Self> {
        let (tx, rx): (Sender<T>, Receiver<T>) = mpsc::channel();

        let worker = thread::spawn(move || {
            <Self as StreamWorker<T>>::worker(StreamSource::from(input), tx);
        });

        Ok(<Self as StreamReader<T, R>>::new(worker, rx))
    }

    fn new(worker: JoinHandle<()>, rx: Receiver<T>) -> Self;
}
