use std::str;

use bytes::Bytes;

/// Represents a value in a KV backend.
///
/// Values are immutable, cheap to clone, and may contain arbitrary binary data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(Bytes);

impl Value {
    #[inline]
    pub fn new(data: impl Into<Bytes>) -> Self {
        Self(data.into())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_bytes())
    }
}

impl<T> From<T> for Value
where
    T: Into<Bytes>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<[u8]> for Value {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<Bytes> for Value {
    #[inline]
    fn as_ref(&self) -> &Bytes {
        &self.0
    }
}