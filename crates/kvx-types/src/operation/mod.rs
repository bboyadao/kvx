mod delete;
mod get;
mod put;

pub use delete::Delete;
pub use get::Get;
pub use put::Put;

/// A backend operation.
///
/// Every operation defines the type it returns.
///
/// Examples:
///
/// Get  -> Option<Value>
/// Put  -> ()
/// Scan -> Stream<Entry>
pub trait Operation {
    type Output;
}