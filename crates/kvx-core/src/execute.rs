use std::future::Future;

use kvx_types::Operation;

use crate::{
    Connection,
    KvError,
};

/// Executes a specific operation.
///
/// Backends implement this trait once per supported operation.
pub trait Execute<O>: Connection
where
    O: Operation,
{
    fn execute(
        &self,
        operation: O,
    ) -> impl Future<Output = Result<O::Output, KvError>> + Send;
}