use kvx_core::{
    Backend,
    Execute,
    Executor,
    KvxError,
};


pub struct Connection<B>
where
    B: Backend,
{
    executor: Executor<B>,
}


impl<B> Connection<B>
where
    B: Backend,
{

    pub fn new(
        backend: B,
    ) -> Self {

        Self {
            executor: Executor::new(backend),
        }

    }


    pub fn executor(
        &self,
    ) -> &Executor<B> {

        &self.executor

    }


    pub async fn execute<O>(
        &self,
        operation: O,
    ) -> Result<O::Output, KvxError>
    where
        O: Execute,
    {
        self.executor
            .execute(operation)
            .await
    }

}