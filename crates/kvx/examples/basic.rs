use kvx::KVX;


#[tokio::main]
async fn main() {

    let client = KVX::connect(
        "redis://127.0.0.1/"
    )
    .await
    .unwrap();


    client
        .set(
            "hello",
            "world"
        )
        .await
        .unwrap();


    let v = client
        .get("hello")
        .await
        .unwrap();


    println!("{:?}", v);
}