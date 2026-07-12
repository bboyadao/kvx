use crate::{Key, Operation, Value};

#[derive(Debug, Clone)]
pub struct Put {
    key: Key,
    value: Value,
}

impl Put {
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

impl Operation for Put {
    type Output = ();
}