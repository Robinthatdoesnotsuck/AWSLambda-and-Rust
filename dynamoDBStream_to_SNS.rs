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
use rusoto_sns::{PublishInput, Sns, SnsClient};

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
    publish_sns(e);
    Ok(CustomOutput {
        message: format!("Processing a dynamoDBStream, {}!", serde_json::to_string(&e.records).unwrap()),
    })
}

#[tokio::main]
async fn publish_sns(e: CustomEvent)
{
    let vec_len = &e.records.len();
    let client = SnsClient::new(Region::Whatever region you want);
    for n in 0..*vec_len
    {
        let mut message_attributes = HashMap::new();
        message_attributes.insert(String::from("Attribute"), MessageAttributeValue{
            string_value:    Some(data.to_string()),
            binary_value: None,
            data_type: "String".to_string(), //most of the time is string
         });
        // this just sent the entire dynamo db stream to a sns 
        let sns_message = PublishInput {
            message: &e.records[n],
            topic_arn: Some("SNS ARN of choice".to_string()),
            message_attributes: Some(message_attributes),
            ..Default::default()
        };
        match client.publish(sns_message).await {
            Ok(output) => {
                println!("Message sent with ID: {:?}", output);
            },
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}