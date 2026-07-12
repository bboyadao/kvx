use redis::AsyncCommands;

use kvx_core::{
    Handler,
    KvError,
};

use kvx_types::Delete;

use crate::RedisClient;

impl Handler<Delete> for RedisClient {
    async fn handle(
        &self,
        operation: Delete,
    ) -> Result<bool, KvError> {
        let mut conn = self.connection().clone();

        let deleted: usize = conn
            .del(operation.key().as_bytes())
            .await
            .map_err(|e| KvError::Backend(e.to_string()))?;

        Ok(deleted > 0)
    }
}