use crate::{
    Backend,
    Execute,
    KvxError,
};

pub struct Executor<B>
where
    B: Backend,
{
    backend: B,
}

impl<B> Executor<B>
where
    B: Backend,
{
    pub fn new(
        backend: B,
    ) -> Self {
        Self { backend }
    }

    pub fn backend(
        &self,
    ) -> &B {
        &self.backend
    }

    pub async fn execute<O>(
        &self,
        operation: O,
    ) -> Result<O::Output, KvxError>
    where
        O: Execute,
    {
        let request =
            operation.into_request();

        let response =
            self.backend
                .execute(request)
                .await?;

        O::from_response(response)
    }
}