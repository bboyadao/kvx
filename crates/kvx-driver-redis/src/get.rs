use async_trait::async_trait;
use redis::AsyncCommands;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::{
    Get,
    Value,
};

use crate::RedisClient;


#[async_trait(?Send)]
impl Handler<Get> for RedisClient {

    type Output = Option<Value>;


    async fn handle(
        &self,
        operation: Get,
    ) -> Result<Self::Output, KvxError> {


        let mut conn =
            self.connection();


        let value: Option<Vec<u8>> =
            conn
            .get(
                operation.key().as_bytes()
            )
            .await
            .map_err(|e|
                KvxError::Backend(
                    e.to_string()
                )
            )?;


        Ok(value.map(Value::from))
    }
}