use worker::kv::KvStore;

pub struct WorkersDriver {
    kv: KvStore,
}

impl WorkersDriver {
    #[inline]
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    #[inline]
    pub fn inner(&self) -> &KvStore {
        &self.kv
    }

    #[inline]
    pub fn into_inner(self) -> KvStore {
        self.kv
    }
}