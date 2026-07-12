use std::future::Future;

use crate::{
    Connection,
    KvError,
};

use kvx_types::Operation;

/// Execute operations against a KV backend.
pub trait Kv: Connection {
    fn execute<O>(
        &self,
        operation: O,
    ) -> impl Future<Output = Result<O::Output, KvError>> + Send
    where
        O: Operation + Send;
}