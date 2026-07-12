use crate::{Key, Operation, Value};

#[derive(Debug, Clone)]
pub struct Get {
    key: Key,
}

impl Get {
    pub fn new(key: impl Into<Key>) -> Self {
        Self { key: key.into() }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl Operation for Get {
    type Output = Option<Value>;
}