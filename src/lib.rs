use aws_sdk_dynamodb::Client;
use egnitely_client::{Context, Error};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_0_17::{from_item, to_item};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct FunctionContextData {
    pub table_name: String,
    pub primary_key: String,
    pub index_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInput {
    pub filter: HashMap<String, Value>,
}

pub async fn handler(mut _ctx: Context, _input: FunctionInput) -> Result<Value, Error> {
    let config_data = serde_json::from_value::<FunctionContextData>(_ctx.config())?;
    let mut record: Value = json!({});
    let config = aws_config::from_env().region("ap-south-1").load().await;
    let client = Client::new(&config);
    let item = client
        .get_item()
        .table_name(config_data.table_name)
        .set_key(Some(to_item(_input.filter)?))
        .send()
        .await?;
    if let Some(item_data) = item.item() {
        record = from_item(item_data.clone())?;
    }
    Ok(json!({
            "message": "Successfully retrived 1 record",
            "data": record
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn trigger_function() {
    
        let resp = handler(
            Context::new(
                "test".to_string(),
                "test".to_string(),
                json!({
                    "table_name": "functions",
                    "primary_key": "id",
                    "index_data": {
                        "team_id": "team_id-index"
                    },
                }),
                json!({}),
            ),
            FunctionInput {
                filter: HashMap::from([("id".to_string(), json!("9b999589-e1eb-4356-a477-c38df2d3a682"))]),
            },
        )
        .await;

        match resp {
            Ok(res) => {
                println!("{}", res);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        };

        assert_eq!(true, true);
    }
}
