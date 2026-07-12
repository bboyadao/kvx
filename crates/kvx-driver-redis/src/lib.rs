mod client;
mod delete;
mod driver;
mod get;
mod options;
mod set;
pub use driver::RedisDriver;
pub use options::RedisOptions;
pub use client::RedisClient;


use anyhow::Result;
use async_trait::async_trait;
use kvx_core::Connection;
use redis::AsyncCommands;


pub struct RedisConnection {
    client: redis::Client,
}


impl RedisConnection {

    pub async fn connect(
        url: &str
    ) -> Result<Self> {

        let client = redis::Client::open(url)?;

        Ok(Self {
            client,
        })
    }


    async fn connection(
        &self
    ) -> Result<redis::aio::MultiplexedConnection> {

        Ok(
            self.client
                .get_multiplexed_async_connection()
                .await?
        )
    }
}



#[async_trait]
impl Connection for RedisConnection {

    async fn get(
        &self,
        key: &str,
    ) -> Result<Option<Vec<u8>>> {

        let mut conn = self.connection().await?;

        let value = conn
            .get(key)
            .await?;

        Ok(value)
    }


    async fn set(
        &self,
        key: &str,
        value: &[u8],
    ) -> Result<()> {

        let mut conn = self.connection().await?;

        let _: () = conn
            .set(key, value)
            .await?;

        Ok(())
    }

    async fn delete(
        &self,
        key: &str,
    ) -> Result<()> {

        let mut conn = self.connection().await?;

        let _: usize = conn
            .del(key)
            .await?;

        Ok(())
    }
}