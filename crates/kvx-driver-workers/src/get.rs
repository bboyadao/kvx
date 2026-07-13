use async_trait::async_trait;

use kvx_core::{
    Handler,
    KvxError,
};

use kvx_types::{
    Get,
    Value,
};

use crate::WorkersClient;

#[async_trait(?Send)]
impl Handler<Get> for WorkersClient {
    type Output = Option<Value>;

    async fn handle(
        &self,
        operation: Get,
    ) -> Result<Self::Output, KvxError> {

        let key = operation
            .key()
            .as_str()
            .map_err(|_| KvxError::InvalidKeyEncoding)?;

        let value = self
            .kv()
            .get(key)
            .bytes()
            .await
            .map_err(|e| KvxError::Backend(e.to_string()))?;

        Ok(value.map(Value::from))
    }
}