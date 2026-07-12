use anyhow::{anyhow, Result};

use kvx_core::Connection;
use kvx_driver_redis::RedisConnection;


pub async fn connect(
    url: &str,
) -> Result<Box<dyn Connection>> {

    if url.starts_with("redis://") {

        let connection =
            RedisConnection::connect(url)
            .await?;


        return Ok(Box::new(connection));
    }


    Err(anyhow!(
        "unsupported kvx connection url: {}",
        url
    ))
}