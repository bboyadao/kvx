use async_trait::async_trait;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::Delete;

use crate::WorkersClient;

#[async_trait(?Send)]
impl Handler<Delete> for WorkersClient {
    type Output = bool;

    async fn handle(
        &self,
        operation: Delete,
    ) -> Result<Self::Output, KvxError> {

        let key = operation
            .key()
            .as_str()
            .map_err(|_| KvxError::InvalidKeyEncoding)?;

        self.kv()
            .delete(key)
            .await
            .map_err(|e| KvxError::Backend(e.to_string()))?;

        // Cloudflare Workers KV does not report whether the key existed.
        Ok(true)
    }
}