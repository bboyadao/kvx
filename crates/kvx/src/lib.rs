use anyhow::{anyhow, Result};
mod connection;

pub use connection::KVX;
use kvx_core::Executor;

use kvx_driver_redis::{
    RedisClient,
    RedisOptions,
};


pub use kvx_core::{
    Execute,
    KvxError,
};

pub use kvx_types::{
    Delete,
    Get,
    Set,
    Value,
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