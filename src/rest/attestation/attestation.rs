use crate::utils::request::{make_auth_request, AuthRequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct AttestationClient {
    base_url: String,
}

impl AttestationClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn create_vin_vc(
        &self,
        token_id: &str,
        force: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vc/vin/{}", token_id);
        let mut query_params: HashMap<String, String> = HashMap::new();
        query_params.insert("force".to_string(), force.to_string());

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.to_string(),
            path,
            query_params: Some(query_params),
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn create_pom_vc(&self, token_id: &str, data: HashMap<String, Value>) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vc/pom/{}", token_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.to_string(),
            path,
            query_params: None,
            body: Some(data),
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }
}
