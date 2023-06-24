use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid tick rate for timeout: {0:?} / {1:?}")]
    InvalidTickRate(Duration, Duration),
    #[error("failed to calculate tick as {0:?} / {1}")]
    FailedToCalculateTick(Duration, u32),

    #[error(transparent)]
    Notify(#[from] notify::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
