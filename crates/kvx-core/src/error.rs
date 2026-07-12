use thiserror::Error;


#[derive(Debug, Error)]
pub enum KvxError {

    #[error("backend error: {0}")]
    Backend(String),


    #[error("internal error: {0}")]
    Internal(String),

}