use kvx::{
    connect,
    Delete,
    Execute,
    Get,
    Set,
};


#[tokio::main]
async fn main()
-> anyhow::Result<()> {


    let db =
        connect(
            "redis://127.0.0.1/"
        )
        .await?;



    db.execute(
        Set::new(
            "hello",
            "world",
        )
    )
    .await?;



    let value =
        db.execute(
            Get::new(
                "hello"
            )
        )
        .await?;


    println!(
        "before delete: {value:?}"
    );



    let deleted =
        db.execute(
            Delete::new(
                "hello"
            )
        )
        .await?;


    println!(
        "deleted: {deleted}"
    );



    let value =
        db.execute(
            Get::new(
                "hello"
            )
        )
        .await?;


    println!(
        "after delete: {value:?}"
    );



    Ok(())
}