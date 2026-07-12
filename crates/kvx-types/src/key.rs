use bytes::Bytes;

/// Represents a key in a KV backend.
///
/// Keys are immutable and cheap to clone.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(Bytes);

impl Key {
    pub fn new(data: impl Into<Bytes>) -> Self {
        Self(data.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T> From<T> for Key
where
    T: Into<Bytes>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}