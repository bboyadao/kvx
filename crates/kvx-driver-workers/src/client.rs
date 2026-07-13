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
}