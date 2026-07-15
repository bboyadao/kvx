mod connection;

pub use connection::Connection;
pub use kvx_core::{
    Backend,
    BackendFactory,
    Execute,
    Executor,
    KvxError,
    Operation,
};

use anyhow::{anyhow, Result};

use kvx_driver_redis::{
    RedisClient,
    RedisOptions,
};


pub async fn connect(
    url: &str,
)
-> Result<Connection<RedisClient>> {

    if url.starts_with("redis://") {

        let client =
            RedisClient::connect(
                RedisOptions::new(url)
            )
            .await?;

        return Ok(
            Connection::new(client)
        );
    }


    Err(
        anyhow!(
            "unsupported kvx connection url: {}",
            url
        )
    )
}