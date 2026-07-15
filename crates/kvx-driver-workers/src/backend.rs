use async_trait::async_trait;

use kvx_core::{
    Backend,
    Request,
    Response,
    KvxError,
};

use crate::WorkersClient;


#[async_trait(?Send)]
impl Backend for WorkersClient {

    async fn execute(
        &self,
        request: Request,
    ) -> Result<Response, KvxError> {

        match request {

            Request::Get(operation) => {

                let value =
                    self
                        .get(operation)
                        .await?;

                Ok(
                    Response::Value(value)
                )

            }


            Request::Set(operation) => {

                self
                    .set(operation)
                    .await?;

                Ok(
                    Response::Empty
                )

            }


            Request::Delete(operation) => {

                let deleted =
                    self
                        .delete(operation)
                        .await?;

                Ok(
                    Response::Bool(deleted)
                )

            }

        }

    }

}