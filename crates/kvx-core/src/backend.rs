use crate::Connection;

/// Marker trait implemented by every backend connection.
///
/// A backend may implement additional capability traits
/// such as Kv, Scan, Watch...
pub trait Backend: Connection {}