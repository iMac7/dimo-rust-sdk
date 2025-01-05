use crate::utils::request::{make_auth_request, AuthRequestParams};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct Telemetry {
    base_url: String,
}

impl Telemetry {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn query(&self, query: &str) -> Result<Value, Box<dyn Error>> {
        let mut body = HashMap::new();
        body.insert("query".to_string(), Value::String(query.to_string()));

        let params = AuthRequestParams {
            method: reqwest::Method::POST,
            base_url: self.base_url.clone(),
            path: "/telemetry".to_string(),
            query_params: None,
            body: Some(body),
            headers: None,
            token_type: "vehicle".to_string(),
        };

        make_auth_request(params).await
    }

    pub async fn get_latest_signals(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let query = format!(
            r#"
            query {{
                SignalsLatest(tokenID: "{}") {{
                    powertrainTransmissionTravelledDistance {{
                        timestamp
                        value
                    }}
                    exteriorAirTemperature {{
                        timestamp
                        value
                    }}
                    speed {{
                        timestamp
                        value
                    }}
                    powertrainType {{
                        timestamp
                        value
                    }}
                }}
            }}
            "#,
            token_id
        );

        self.query(&query).await
    }
}
