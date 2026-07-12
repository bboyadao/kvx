use crate::RedisOptions;

/// Redis client.
///
/// Currently only stores configuration.
/// The transport layer will be introduced later.
#[derive(Debug, Clone)]
pub struct RedisClient {
    options: RedisOptions,
}

impl RedisClient {
    pub fn new(options: RedisOptions) -> Self {
        Self { options }
    }

    pub fn options(&self) -> &RedisOptions {
        &self.options
    }
}