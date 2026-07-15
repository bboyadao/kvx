pub use kvx_core::{
    Execute,
    Executor,
    KvxError,
};

pub use kvx_core::{
    Delete,
    Get,
    Set,
    Value,
};


use anyhow::{anyhow, Result};

use kvx_driver_redis::{
    RedisClient,
    RedisOptions,
};


pub async fn connect(
    url: &str,
)
-> Result<Executor<RedisClient>> {

    if url.starts_with("redis://") {

        let client =
            RedisClient::connect(
                RedisOptions::new(url)
            )
            .await?;

        return Ok(
            Executor::new(client)
        );
    }


    Err(
        anyhow!(
            "unsupported kvx connection url: {}",
            url
        )
    )
}