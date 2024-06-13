use thiserror::Error;

#[derive(Error, Debug)]
pub enum MactimeError {
    #[error("An IO Error has occurred: {0}")]
    IoError(std::io::Error)
}

impl From<std::io::Error> for MactimeError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}