use clap::{
    Parser,
    Subcommand,
};

use kvx::{
    connect,
    Delete,
    Execute,
    Get,
    Set,
};


#[derive(Parser)]
struct Cli {

    url: String,


    #[command(subcommand)]
    command: Command,

}



#[derive(Subcommand)]
enum Command {

    Get {
        key: String,
    },


    Set {
        key: String,
        value: String,
    },


    Delete {
        key: String,
    },

}



#[tokio::main]
async fn main()
-> anyhow::Result<()> {


    let cli =
        Cli::parse();



    let db =
        connect(
            &cli.url
        )
        .await?;



    match cli.command {


        Command::Get {
            key,
        } => {


            let value =
                db.execute(
                    Get::new(key)
                )
                .await?;


            println!(
                "{value:?}"
            );

        }



        Command::Set {
            key,
            value,
        } => {


            db.execute(
                Set::new(
                    key,
                    value,
                )
            )
            .await?;


        }



        Command::Delete {
            key,
        } => {


            let deleted =
                db.execute(
                    Delete::new(key)
                )
                .await?;


            println!(
                "deleted: {deleted}"
            );

        }

    }


    Ok(())

}