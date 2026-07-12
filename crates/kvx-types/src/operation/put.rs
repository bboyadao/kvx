use crate::{
    Key,
    Value,
};

use super::Operation;

/// Store a value.
#[derive(Debug, Clone)]
pub struct Put {
    pub key: Key,
    pub value: Value,
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
}

impl Operation for Put {
    type Output = ();
}