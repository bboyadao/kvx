use redis::{
    aio::MultiplexedConnection,
    Client,
};

use crate::RedisOptions;

/// Connected Redis client.
pub struct RedisClient {
    connection: MultiplexedConnection,
}

impl RedisClient {
    /// Connect to Redis.
    pub async fn connect(
        options: RedisOptions,
    ) -> redis::RedisResult<Self> {
        let client = Client::open(options.url)?;

        let connection = client
            .get_multiplexed_async_connection()
            .await?;

        Ok(Self { connection })
    }

    pub(crate) fn connection(&self) -> &MultiplexedConnection {
        &self.connection
    }
}