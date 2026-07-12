use crate::Key;

use super::Operation;

/// Delete a key.
#[derive(Debug, Clone)]
pub struct Delete {
    pub key: Key,
}

impl Delete {
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
        }
    }
}

impl Operation for Delete {
    type Output = bool;
}