use crate::{Key, Operation};

#[derive(Debug, Clone)]
pub struct Delete {
    key: Key,
}

impl Delete {
    pub fn new(key: impl Into<Key>) -> Self {
        Self { key: key.into() }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Operation for Delete {
    type Output = bool;
}