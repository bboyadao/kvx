use std::future::Future;

use kvx_types::Operation;

use crate::{
    Connection,
    KvError,
};

/// Handles a single operation.
///
/// A backend implements this trait once for every
/// supported operation.
///
/// Example:
///
/// impl Handler<Get> for RedisConnection {}
///
/// impl Handler<Put> for RedisConnection {}
pub trait Handler<O>: Connection
where
    O: Operation,
{
    fn handle(
        &self,
        operation: O,
    ) -> impl Future<Output = Result<O::Output, KvError>> + Send;
}