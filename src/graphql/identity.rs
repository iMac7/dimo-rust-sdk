use crate::utils::request::{make_request, RequestParams};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct Identity {
    base_url: String,
}

impl Identity {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn query(&self, query: &str) -> Result<Value, Box<dyn Error>> {
        let mut body = HashMap::new();
        body.insert("query".to_string(), Value::String(query.to_string()));

        let params = RequestParams {
            method: reqwest::Method::POST,
            base_url: self.base_url.clone(),
            path: "".to_string(),
            query_params: None,
            body: Some(body),
            headers: None,
        };

        make_request(params).await
    }

    pub async fn count_dimo_vehicles(&self) -> Result<Value, Box<dyn Error>> {
        let query = "
        {
            vehicles (first: 10) {
                totalCount
            }
        }
        ";
        self.query(query).await
    }

    pub async fn list_vehicle_definitions_per_address(
        &self,
        address: &str,
        limit: Option<i32>,
    ) -> Result<Value, Box<dyn Error>> {
        let limit = limit.unwrap_or(10);

        let query = format!(
            r#"
            {{
                vehicles(filterBy: {{owner: "{address}"}}, first: {limit}) {{
                    nodes {{
                        aftermarketDevice {{
                            tokenId
                            address
                        }}
                        syntheticDevice {{
                            address
                            tokenId
                        }}
                        definition {{
                            make
                            model
                            year
                        }}
                    }}
                }}
            }}
            "#,
            address = address,
            limit = limit
        );

        self.query(&query).await
    }
}
