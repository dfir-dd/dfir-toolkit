use std::{io::Read, sync::mpsc::Sender};

use crate::filter::Provider;

pub(crate) trait StreamWorker<T>: Provider<T, ()> {
    fn worker<R: Read + Send>(input: R, tx: Sender<T>)
    where
        T: Send;
}
