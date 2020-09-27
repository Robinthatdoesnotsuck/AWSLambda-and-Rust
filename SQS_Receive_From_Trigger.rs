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

#[derive(Deserialize, Clone,Serialize)]
struct CustomEvent {
    #[serde(rename = "Records")]
    records: Vec<Map<String, Value>>,
}
#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>>
{
    simple_logger::init_with_level(log::Level::Info)?;
    lambda::lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent,c: lambda::Context) -> Result<CustomOutput, HandlerError> 
{
    
    let vec_len = e.records.len();
    
    for n in 0..vec_len
    {
        // This is to parse from string to json if you send your messages via an sns first
        let sqs = &e.records[n];
        let json_message_parse: Value = serde_json::from_str(sqs["body"].as_str().unwrap()).unwrap();
        let json_dynamodb_parse: Value = serde_json::from_str(json_message_parse["Message"].as_str().unwrap()).unwrap();
    }
    Ok(CustomOutput {
        message: format!("Processing a sqs, {}!", serde_json::to_string(&e.records).unwrap()),
    })
}