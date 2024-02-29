pub mod dadjokes;

use anyhow::Error;
use aws_sdk_dynamodb::operation::create_table::CreateTableOutput;
use aws_sdk_dynamodb::operation::delete_table::DeleteTableOutput;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType, AttributeValue
};
use aws_sdk_dynamodb::Client;

pub async fn create_table(client: &Client, table: &String, key: &String) -> Result<CreateTableOutput, Error>{
    let ad = AttributeDefinition::builder()
        .attribute_name(key)
        .attribute_type(ScalarAttributeType::S)
        .build()?;

    let ks = KeySchemaElement::builder()
        .attribute_name(key)
        .key_type(KeyType::Hash)
        .build()?;

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(50)
        .write_capacity_units(50)
        .build()?;

    let create_table_response = client
        .create_table()
        .table_name(table)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await;

    match create_table_response {
        Ok(out) => {
            println!("Added table {} with key {}", table, key);
            Ok(out)
        }
        Err(e) => {
            eprintln!("Got an error creating table:");
            eprintln!("{}", e);
            Err(Error::new(e))
        }
    }
}

pub async fn delete_table(client: &Client, table: &String) -> Result<DeleteTableOutput, Error> {
    let resp = client.delete_table().table_name(table).send().await;

    match resp {
        Ok(out) => {
            println!("Deleted table: {}", table);
            Ok(out)
        }
        Err(e) => Err(Error::new(e)),
    }
}


pub async fn add_dadjoke(client: &Client, joke: dadjokes::DadJoke, table: &String) -> Result<(), Error> {
    let id_av = AttributeValue::S(joke.id.into());
    let joke_av = AttributeValue::S(joke.joke);
    let punchline_av = AttributeValue::S(joke.punchline);
    let category_av = AttributeValue::S(joke.category);

    let request = client
        .put_item()
        .table_name(table)
        .item("id", id_av)
        .item("joke", joke_av)
        .item("punchline", punchline_av)
        .item("category", category_av);

    //println!("Executing request [{request:?}] to add item...");

    let _resp = request.send().await?;

    //let attributes = resp.attributes().unwrap();

    //let joke = attributes.get("joke").cloned();
    //let punchline = attributes.get("punchline").cloned();
    //let category = attributes.get("category").cloned();
    //let id = attributes.get("id").cloned();

    //println!(
    //    "Added joke {:?}, with punchline: {:?}, of cateogory {:?}. And it has this ID: {:?}",
    //    joke, punchline, category, id
    //);

    Ok(())
}

pub async fn jokes_in_category(client: &Client, table_name: &String,category: String) -> Result<Vec<dadjokes::DadJoke>, Error> {
    let results = client
        .query()
        .table_name(table_name)
        .index_name("category-index")
        .key_condition_expression("category = :cat")
        .expression_attribute_values(":cat", AttributeValue::S(category))
        .send()
        .await?;

    if let Some(items) = results.items {
        let jokes = items.iter().map(|v| v.into()).collect();
        Ok(jokes)
    } else {
        Ok(vec![])
    }
}




