#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    AsyncWatcher(#[from] async_watcher::error::Error),
    #[error(transparent)]
    Globset(#[from] globset::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;
