use redis::AsyncCommands;
use crate::client::RedisClient;

use kvx_core::{
    Handler,
    KvError,
};

use kvx_types::{
    Get,
    Value,
};



impl Handler<Get> for RedisClient {
    async fn handle(
        &self,
        operation: Get,
    ) -> Result<Option<Value>, KvError> {
        let mut conn = self.connection().clone();

        let value: Option<Vec<u8>> = conn
            .get(operation.key().as_bytes())
            .await
            .map_err(|e| KvError::Backend(e.to_string()))?;

        Ok(value.map(Value::from))
    }
}