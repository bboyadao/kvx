use crate::{
    Key,
    Value,
};

use super::Operation;

/// Get the value associated with a key.
#[derive(Debug, Clone)]
pub struct Get {
    pub key: Key,
}

impl Get {
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
        }
    }
}

impl Operation for Get {
    type Output = Option<Value>;
}