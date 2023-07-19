use thiserror::Error;


#[derive(Debug, Error)]
pub enum SessionEventError {
    #[error("this event does not belong to a session")]
    NoSessionEvent,

    #[error("some other error has occurred")]
    WrappedError(anyhow::Error),
}

impl From<anyhow::Error> for SessionEventError {
    fn from(value: anyhow::Error) -> Self {
        Self::WrappedError(value)
    }
}
