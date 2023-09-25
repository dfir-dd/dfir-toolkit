use std::{io::Read, sync::mpsc::Sender};

use crate::apps::mactime2::filter::Provider;

pub(crate) trait StreamWorker<T>: Provider<T, ()> {
    fn worker<R: Read + Send>(input: R, tx: Sender<T>) where T: Send;
}