use worker::*;
use kvx_driver_workers::WorkersDriver;

#[event(fetch)]
async fn fetch(
    _: Request,
    env: Env,
    _: Context,
) -> Result<Response> {

    let kv = env.kv("USERS")?;

    let store = WorkersDriver::new(kv);

    store.put(
        "hello",
        b"world".to_vec(),
    ).await.unwrap();

    let value = store.get("hello").await.unwrap();

    Response::ok(format!("{:?}", value))
}