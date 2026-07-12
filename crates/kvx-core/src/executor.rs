use async_trait::async_trait;

use crate::{
    Execute,
    Handler,
    KvxError,
};



pub struct Executor<C> {

    client: C,

}



impl<C> Executor<C> {

    pub fn new(
        client: C,
    ) -> Self {

        Self {
            client,
        }

    }


    pub fn client(
        &self,
    ) -> &C {

        &self.client

    }

}



#[async_trait]
impl<C, O> Execute<O> for Executor<C>
where
    C: Handler<O> + Send + Sync,
    O: Send + Sync + 'static,
{

    type Output = C::Output;


    async fn execute(
        &self,
        operation: O,
    )
    -> Result<Self::Output, KvxError> {


        self.client
            .handle(operation)
            .await

    }

}