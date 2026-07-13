use async_trait::async_trait;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::Set;

use crate::WorkersClient;

#[async_trait(?Send)]
impl Handler<Set> for WorkersClient {
    type Output = ();

    async fn handle(
        &self,
        operation: Set,
    ) -> Result<Self::Output, KvxError> {

        let key = operation
            .key()
            .as_str()
            .map_err(|_| KvxError::InvalidKeyEncoding)?;

        self.kv()
            .put(key, operation.value().as_bytes())
            .map_err(|e| KvxError::Backend(e.to_string()))?
            .execute()
            .await
            .map_err(|e| KvxError::Backend(e.to_string()))?;

        Ok(())
    }
}