use thiserror::Error;

/// Common errors returned by KVX.
#[derive(Debug, Error)]
pub enum KvError {
    #[error("key not found")]
    NotFound,

    #[error("unsupported operation")]
    Unsupported,

    #[error("connection failed")]
    Connection,

    #[error("{0}")]
    Backend(String),
}