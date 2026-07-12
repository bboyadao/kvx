use crate::{
    Key,
    Metadata,
    Value,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: Key,
    pub value: Value,
    pub metadata: Metadata,
}