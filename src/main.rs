pub mod utils;

use clap::Parser;

use anyhow::Result;
use utils::configure_aws;
use dadnamodb::create_table;
use dadnamodb::delete_table;
use dadnamodb::add_dadjoke;
use dadnamodb::dadjokes::load_jokes_from_file;
use dadnamodb::jokes_in_category;

use indicatif::ProgressBar;

#[tokio::main]
async fn main() -> Result<()>{
    // parsing arguments
    let arguments  = utils::Args::parse();
    // configuring the SDK
    let config =  configure_aws(String::from("us-west-2")).await;
    // setup the bedrock-runtime client
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    // are we creating a table?
    if arguments.initialize {
        println!("========================================");
        println!("Initializing a new table");
        println!("========================================");
        create_table(&dynamodb_client, &arguments.table_name, &String::from("id")).await?;
    };

    if arguments.rm {
        println!("========================================");
        println!("Deleting the table {}", arguments.table_name);
        println!("========================================");
        delete_table(&dynamodb_client, &arguments.table_name).await?;
    }
    if arguments.populate {
        println!("========================================");
        println!("Populating the table {}", arguments.table_name);
        println!("========================================");
        let jf = load_jokes_from_file("categorized_jokes.json")?;
        // TODO: handle the usize into u64 with .try_into()
        let bar = ProgressBar::new(jf.jokes.len() as u64);
        for joke in jf.jokes {
            bar.inc(1);
            add_dadjoke(&dynamodb_client, joke, &arguments.table_name).await?;
        }
    }

    match arguments.query {
        Some(q) => {
            // run query
            let qjokes = jokes_in_category(&dynamodb_client, &arguments.table_name, q).await?;
            //println!("{:#?}",qjokes);
            for joke in qjokes {
                joke.say();
            }
        },
        None => println!("There was no query set.")
    }

    Ok(())
}
