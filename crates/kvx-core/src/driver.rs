/// Backend driver.
///
/// A driver only defines the types required
/// to create a backend client.
pub trait Driver {
    type Client;
    type Options;
}