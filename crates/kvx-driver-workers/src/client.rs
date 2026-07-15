use kvx_core::{Delete, Get, KvxError, Set, Value};

use worker::kv::KvStore;

pub struct WorkersClient {
    kv: KvStore,
}

impl WorkersClient {
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    pub(crate) fn kv(&self) -> &KvStore {
        &self.kv
    }

    pub(crate) async fn get(&self, operation: Get) -> Result<Option<Value>, KvxError> {
        let key = operation
            .key()
            .as_str()
            .map_err(|_| KvxError::InvalidKeyEncoding)?;
        let value = self
            .kv()
            .get(key)
            .text()
            .await
            .map_err(|e| KvxError::Backend(e.to_string()))?;

        Ok(value.map(Value::from))
    }

    pub(crate) async fn set(&self, operation: Set) -> Result<(), KvxError> {
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

    pub(crate) async fn delete(&self, operation: Delete) -> Result<bool, KvxError> {
        let key = operation
            .key()
            .as_str()
            .map_err(|_| KvxError::InvalidKeyEncoding)?;

        self.kv()
            .delete(key)
            .await
            .map_err(|e| KvxError::Backend(e.to_string()))?;

        // Workers KV không trả về trạng thái key có tồn tại hay không
        Ok(true)
    }
}
