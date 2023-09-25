use thiserror::Error;

#[derive(Error, Debug)]
pub enum MactimeError {
    #[error("ambigious file name: '{0}'")]
    AmbiguousFilename(String),
}