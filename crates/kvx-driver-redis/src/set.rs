use redis::AsyncCommands;

use kvx_core::{
    Handler,
    KvError,
};
use kvx_types::Set;
use crate::RedisClient;

impl Handler<Set> for RedisClient {
    async fn handle(
        &self,
        operation: Set,
    ) -> Result<(), KvError> {
        let mut conn = self.connection().clone();

        let _: () = conn
    .set(
        operation.key().as_bytes(),
        operation.value().as_bytes(),
    )
    .await
    .map_err(|e| KvError::Backend(e.to_string()))?;

        Ok(())
    }
}