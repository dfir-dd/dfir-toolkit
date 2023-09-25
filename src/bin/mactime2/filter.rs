use std::sync::mpsc::{Sender, Receiver};

use chrono_tz::Tz;

#[derive(Copy, Clone)]
pub struct RunOptions {
    pub strict_mode: bool,
    pub src_zone: Tz
}

pub trait Provider<To, R>: Joinable<R> {
    fn get_receiver(&mut self) -> Receiver<To>;
}

pub trait Consumer<From> {
    fn with_receiver(previous: Receiver<From>, options: RunOptions) -> Self;
}

pub trait Filter<From, To, R> : Consumer<From> + Provider<To, R> {
    fn worker(reader: Receiver<From>, tx: Sender<To>, options: RunOptions);
}

pub trait Joinable<R> {
    fn join(&mut self) -> std::thread::Result<R>;
}

pub trait Runnable {
    fn run(&mut self);
}

pub trait Sorter<T>: Runnable + Joinable<T> {}