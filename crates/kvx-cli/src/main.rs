use clap::{Parser, Subcommand};
use kvx::connect;


#[derive(Parser)]
struct Cli {

    url: String,


    #[command(subcommand)]
    command: Command,
}



#[derive(Subcommand)]
enum Command {

    Get {
        key: String
    },


    Set {
        key: String,
        value: String
    },

    Delete {
        key: String
    }
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {


    let cli = Cli::parse();


    let db =
        connect(&cli.url)
        .await?;


    match cli.command {


        Command::Get { key } => {

            let value =
                db.get(&key)
                .await?;


            println!(
                "{:?}",
                value
            );
        }


        Command::Set { key, value } => {

            db.set(
                &key,
                value.as_bytes()
            )
            .await?;
        }


        Command::Delete { key } => {

            db.delete(&key)
            .await?;
        }
    }


    Ok(())
}