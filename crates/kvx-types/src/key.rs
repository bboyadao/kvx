use std::str;

use bytes::Bytes;

/// Represents a key in a KV backend.
///
/// Keys are immutable, cheap to clone, and may contain arbitrary binary data.
/// Some backends (e.g. Cloudflare Workers KV) require UTF-8 keys. Use
/// [`Key::as_str`] when interacting with those backends.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(Bytes);

impl Key {
    /// Creates a new key from any type convertible into [`Bytes`].
    #[inline]
    pub fn new(data: impl Into<Bytes>) -> Self {
        Self(data.into())
    }

    /// Returns the raw bytes of the key.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Returns the key as UTF-8.
    ///
    /// This is useful for backends that only support UTF-8 keys, such as
    /// Cloudflare Workers KV.
    #[inline]
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_bytes())
    }
}

impl<T> From<T> for Key
where
    T: Into<Bytes>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<[u8]> for Key {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<Bytes> for Key {
    #[inline]
    fn as_ref(&self) -> &Bytes {
        &self.0
    }
}