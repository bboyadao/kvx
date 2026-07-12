use kvx_core::ExecuteExt;
use kvx_driver_redis::{
    RedisClient,
    RedisOptions,
};
use kvx_types::Get;

use redis::AsyncCommands;

#[tokio::test]
async fn get_existing_key() {
    let options = RedisOptions::new("redis://127.0.0.1/");

    let client = RedisClient::connect(options.clone())
        .await
        .unwrap();

    let redis = redis::Client::open(options.url)
        .unwrap();

    let mut conn = redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let _: () = conn
        .set("hello", "world")
        .await
        .unwrap();

    let value = client
        .execute(Get::new("hello"))
        .await
        .unwrap();

    assert!(value.is_some());
}