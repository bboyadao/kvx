use std::future::Future;

use kvx_types::Operation;

use crate::KvError;

/// Executes one operation.
pub trait Handler<O>
where
    O: Operation,
{
    fn handle(
        &self,
        operation: O,
    ) -> impl Future<Output = Result<O::Output, KvError>> + Send;
}