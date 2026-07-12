use crate::Connection;

/// Marker trait for KV backends.
pub trait Kv: Connection {}