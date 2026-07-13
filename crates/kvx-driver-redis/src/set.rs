use async_trait::async_trait;
use redis::AsyncCommands;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::Set;

use crate::RedisClient;


#[async_trait(?Send)]
impl Handler<Set> for RedisClient {

    type Output = ();


    async fn handle(
        &self,
        operation: Set,
    ) -> Result<Self::Output, KvxError> {


        let mut conn =
            self.connection();


        let _: () =
            conn
            .set(
                operation.key().as_bytes(),
                operation.value().as_bytes(),
            )
            .await
            .map_err(|e|
                KvxError::Backend(
                    e.to_string()
                )
            )?;


        Ok(())
    }
}