# KVX

KVX is a unified Rust interface for multiple key-value databases.

Current status

- ✅ Redis
- 🚧 etcd
- 🚧 Cloudflare Workers KV
- 🚧 Supabase

## Goals

- Unified API
- Zero-cost abstraction
- Async-first
- Backend agnostic
- Type-safe

## Example

```rust
let client = RedisClient::connect(
    RedisOptions::new("redis://127.0.0.1/")
).await?;

client.execute(Put::new("hello", "world")).await?;

let value = client.execute(Get::new("hello")).await?;