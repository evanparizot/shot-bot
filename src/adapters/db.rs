use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
pub struct ShotSaver {
    client: aws_sdk_dynamodb::Client,
}

impl ShotSaver {
    pub fn new(client: aws_sdk_dynamodb::Client) -> Self {
        Self {client: client}
    }

    pub async fn add(&self, name: &str, amount: u8) -> () {
        match self.client.update_item()
            .key("name", AttributeValue::S(name.to_string()))
            .update_expression("ADD Shots :amount")
            .expression_attribute_values("amount", AttributeValue::N(amount.to_string()))
            .return_values(ReturnValue::UpdatedNew).send().await
            {
                Ok(_) => (),
                Err(_e) => ()
            }
        
    }

    pub fn remove(&self, name: String) -> () {

    }
}
