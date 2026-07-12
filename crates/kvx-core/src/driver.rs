use crate::{
    Connection,
    KvError,
};

/// A backend driver.
///
/// A driver is responsible for creating connections.
pub trait Driver {
    /// Active connection type.
    type Connection: Connection;

    /// Connect to backend.
    fn connect(
        uri: impl AsRef<str>,
    ) -> impl Future<
        Output = Result<Self::Connection, KvError>
    > + Send;
}