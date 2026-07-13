use async_trait::async_trait;
use redis::AsyncCommands;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::Delete;

use crate::RedisClient;


#[async_trait(?Send)]
impl Handler<Delete> for RedisClient {

    type Output = bool;


    async fn handle(
        &self,
        operation: Delete,
    ) -> Result<Self::Output, KvxError> {


        let mut conn =
            self.connection();


        let deleted: usize =
            conn
            .del(
                operation.key().as_bytes()
            )
            .await
            .map_err(|e|
                KvxError::Backend(
                    e.to_string()
                )
            )?;


        Ok(deleted > 0)
    }
}