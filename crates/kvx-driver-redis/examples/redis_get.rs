use kvx_core::ExecuteExt;
use kvx_driver_redis::{RedisClient, RedisOptions};
use kvx_types::{Delete, Get, Put};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RedisClient::connect(
        RedisOptions::new("redis://127.0.0.1/")
    )
    .await?;

    client.execute(Put::new("hello", "world")).await?;

    let value = client.execute(Get::new("hello")).await?;
    println!("Before delete: {value:?}");

    let deleted = client.execute(Delete::new("hello")).await?;
    println!("Deleted: {deleted}");

    let value = client.execute(Get::new("hello")).await?;
    println!("After delete: {value:?}");

    Ok(())
}