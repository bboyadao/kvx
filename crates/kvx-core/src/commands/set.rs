use crate::{Key, Operation, Value};

#[derive(Debug, Clone)]
pub struct Set {
    key: Key,
    value: Value,
}

impl Set {
    pub fn new(
        key: impl Into<Key>,
        value: impl Into<Value>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Operation for Set {
    type Output = ();
}