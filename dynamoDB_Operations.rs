
#[macro_use]
extern crate serde_derive;

use lambda_runtime as lambda;
use log;
use simple_logger;
use lambda::error::HandlerError;
use std::error::Error;

use serde_json::map::Map;
use serde_json::value::Value;

use std::collections::HashMap;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, AttributeValue, QueryInput, QueryOutput, UpdateItemInput};

fn main() -> Result<(), Box<dyn Error>>
{
    simple_logger::init_with_level(log::Level::Info)?;
    lambda::lambda!(my_handler);

    Ok(())
}
fn my_handler(e: Map<String, Value>,c: lambda::Context) -> Result<CustomOutput, HandlerError> 
{
    
    insert(e);
    delete(e);
    update(e);
    query(e);
    Ok(CustomOutput {
        message: format!("Processing a dynamoDBStream, {}!", serde_json::to_string(&e.records).unwrap()),
    })
}
#[tokio::main]
async fn insert(sort_key: String)
{
    let mut item_to_insert: HashMap<String, AttributeValue> = HashMap::new();
    item_to_insert.insert(String::from("AN ATTRIBUTE"),AttributeValue{
        n: Some("ATTRIBUTE VALUE"),
        ..Default::default()
    });
    item_to_insert.insert(String::from("partitionKey"), AttributeValue{
               s: Some(/*partition key*/), 
                ..Default::default()
            });
    item_to_insert.insert(String::from("sortKey"), AttributeValue{
               s: Some(/*sort key*/) ,
                ..Default::default()
                
            });
    let put_item = PutItemInput{
                item: item_to_insert,
                table_name: String::from("table name"),
                ..Default::default()
            };
    let client = DynamoDbClient::new(Region::UsEast1);
    match client.put_item(put_item).await {
            Ok(result) => {
                match result.attributes {
                    Some(result) => println!("The element already exists"),
                    None => println!("I finished inserting the element"),
                }
            },
            Err(error) => {
                panic!("Error: {:?}", error);
            },
        };
}
#[tokio::main]
async fn delete(sort_key: String)
{
    let mut item_to_delete: HashMap<String, AttributeValue> = HashMap::new();
    item_to_delete.insert(String::from("partitionKey"), AttributeValue{
               s: Some(/*partition key*/), 
                ..Default::default()
            });
    item_to_delete.insert(String::from("sortKey"), AttributeValue{
               s: Some(/*sort key*/) ,
                ..Default::default()
                
            });
            
    let delete_item = DeleteItemInput{
                key: item_to_insert,
                table_name: String::from("table name"),
                ..Default::default()
            };
    let client = DynamoDbClient::new(Region::UsEast1);
    match client.delete_item(delete_item).await {
            Ok(result) => {
                match result.attributes {
                    Some(result) => println!("The element doesn't exists"),
                    None => println!("I finished deleting the element"),
                }
            },
            Err(error) => {
                panic!("Error: {:?}", error);
            },
        };
}
async fn update(sort_key: String)
{
    let mut data_updater: HashMap<String, AttributeValue> = HashMap::new();
    data_updater.insert(String::from(":AN ATTRIBUTE"),AttributeValue{
        n: Some("ATTRIBUTE VALUE"),
        ..Default::default()
    });
    let mut keys_for_update: HashMap<String, AttributeValue> = HashMap::new();
    keys_for_update.insert(String::from("partitionKey"), AttributeValue{
               s: Some(/*partition key*/), 
                ..Default::default()
            });
            keys_for_update.insert(String::from("sortKey"), AttributeValue{
               s: Some(/*sort key*/) ,
                ..Default::default()
                
            });
    let update = UpdateItemInput{
                expression_attribute_values: Some(data_updater),
                key: keys_for_update,
                update_expression: Some("SET attribute = :attribute and repeat".to_string()),
                table_name: String::from("table name"),
                ..Default::default()
            };
    let client = DynamoDbClient::new(Region::UsEast1);
    match client.update_item(update).await {
            Ok(result) => {
                match result.attributes {
                    Some(result) => println!("The element already exists"),
                    None => println!("I finished updating the element"),
                }
            },
            Err(error) => {
                panic!("Error: {:?}", error);
            },
        };        
}
async fn query(sort_key: String)
{
    let mut query_data: HashMap<String, AttributeValue> = HashMap::new();
    query_data.insert(String::from(":partitionKeyval"), AttributeValue {
        s: Some(/*the parition key*/),
        ..Default::default()
    });
    query_data.insert(String::from(":sortKeyval"), AttributeValue {
        s: Some(/*the sort key*/),
        ..Default::default()
    });
    let client = DynamoDbClient::new(Region::UsEast1);
    let mut data = Vec::new();
    let mut size_of: usize = 0;
    
    let query_input = QueryInput {
        table_name: String::from("TABLE NAME"),
        key_condition_expression: Some(String::from("partitionKey = :partitionKeyval and sortKey = :sortKeyval")),
        expression_attribute_values: Some(query_data),
        ..Default::default()
    };
    let query_output: QueryOutput;

    match client.query(query_input).await {
        Ok(query_output) => {
            println!("Query executed");
            data = query_output.items.unwrap();
            size_of = query_output.count.unwrap() as usize;
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}