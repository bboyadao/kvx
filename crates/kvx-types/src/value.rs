use bytes::Bytes;

/// Represents a value in a KV backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(Bytes);

impl Value {
    pub fn new(data: impl Into<Bytes>) -> Self {
        Self(data.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T> From<T> for Value
where
    T: Into<Bytes>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}