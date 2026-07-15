use kvx::{
    connect,
    Delete,
    Execute,
    Get,
    Set,
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let client =
        connect(
            "redis://127.0.0.1/"
        )
        .await?;


    client
        .execute(
            Set::new(
                "hello",
                "world",
            )
        )
        .await?;


    let value =
        client
            .execute(
                Get::new(
                    "hello"
                )
            )
            .await?;


    println!(
        "GET hello = {:?}",
        value
    );


    client
        .execute(
            Delete::new(
                "hello"
            )
        )
        .await?;


    let value =
        client
            .execute(
                Get::new(
                    "hello"
                )
            )
            .await?;


    println!(
        "GET hello after delete = {:?}",
        value
    );


    Ok(())
}