use crate::utils::request::{make_auth_request, AuthRequestParams};
use reqwest::Method;
use serde_json::Value;
use std::error::Error;

pub struct Valuations {
    base_url: String,
}

impl Valuations {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_valuations(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/valuations", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_instant_offers(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/instant-offer", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_offers(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/offers", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }
}
