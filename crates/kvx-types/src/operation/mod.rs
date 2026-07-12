mod delete;
mod get;
mod set;

pub use delete::Delete;
pub use get::Get;
pub use set::Set;

/// A backend operation.
///
/// Every operation defines the type it returns.
///
/// Examples:
///
/// Get  -> Option<Value>
/// Set  -> ()
/// Scan -> Stream<Entry>
pub trait Operation {
    type Output;
}