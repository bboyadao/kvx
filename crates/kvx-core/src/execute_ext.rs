use std::future::Future;

use kvx_types::Operation;

use crate::{
    Handler,
    KvError,
};

pub trait ExecuteExt: Sized {
    #[inline]
    fn execute<O>(
        &self,
        operation: O,
    ) -> impl Future<Output = Result<O::Output, KvError>> + Send
    where
        O: Operation,
        Self: Handler<O>,
    {
        <Self as Handler<O>>::handle(self, operation)
    }
}

impl<T> ExecuteExt for T {}