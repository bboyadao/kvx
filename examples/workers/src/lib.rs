use worker::*;

use kvx_driver_workers::WorkersClient;


#[event(fetch)]
async fn main(
    _req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {


    let kv = env.kv("MY_KV")?;

    let _client = WorkersClient::new(kv);


    Response::ok("kvx works")
}