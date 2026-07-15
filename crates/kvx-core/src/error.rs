use thiserror::Error;


#[derive(Debug, Error)]
pub enum KvxError {

    #[error("backend error: {0}")]
    Backend(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("key is not valid UTF-8")]
    InvalidKeyEncoding,

    #[error("protocol error: {0}")]
    Protocol(String),
}