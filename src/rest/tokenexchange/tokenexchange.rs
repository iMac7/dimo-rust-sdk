use crate::utils::request::{make_auth_request, AuthRequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct TokenExchange {
    base_url: String,
    contract_address: String,
}

pub struct TokenExchangeParams {
    pub privileges: Vec<i32>,
    pub token_id: i32,
}

impl TokenExchange {
    pub fn new(base_url: &str, contract_address: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            contract_address: contract_address.to_string(),
        }
    }

    pub async fn exchange(
        &self,
        token_id: i32,
        privileges: Vec<i32>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/v1/tokens/exchange".to_string();
        let mut body: HashMap<String, Value> = HashMap::new();
        body.insert(
            "nftContractAddress".to_string(),
            Value::String(self.contract_address.to_string()),
        );
        let privileges_as_values: Vec<Value> = privileges
            .into_iter()
            .map(|num| Value::Number(serde_json::Number::from(num)))
            .collect();
        body.insert("privileges".to_string(), Value::Array(privileges_as_values));
        body.insert("tokenId".to_string(), Value::from(token_id));

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: Some(body),
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }
}
