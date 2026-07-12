/// Represents an active connection to a KV backend.
///
/// Examples:
///
/// - Redis TCP connection
/// - Etcd gRPC channel
/// - Workers KV HTTP client
/// - Supabase REST client
pub trait Connection {}