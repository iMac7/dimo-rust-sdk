use crate::utils::request::{make_auth_request, AuthRequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct Trips {
    base_url: String,
}

impl Trips {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn list(&self, token_id: &str, page: Option<u32>) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vehicle/{}/trips", token_id);

        let mut query_params = HashMap::new();
        if let Some(page) = page {
            query_params.insert("page".to_string(), page.to_string());
        }

        let request_params = AuthRequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: Some(query_params),
            body: None,
            headers: None,
            token_type: "vehicle".to_string(),
        };

        make_auth_request(request_params).await
    }
}
