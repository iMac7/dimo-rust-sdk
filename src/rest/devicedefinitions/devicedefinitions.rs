use crate::utils::request::{make_request, RequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct DeviceDefinitions {
    base_url: String,
}

impl DeviceDefinitions {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn all(&self) -> Result<Value, Box<dyn Error>> {
        let path = "/device-definitions/all".to_string();

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn get_by_mmy(
        &self,
        make: &str,
        model: &str,
        year: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/device-definitions".to_string();
        let mut query_params = HashMap::new();
        query_params.insert("make".to_string(), make.to_string());
        query_params.insert("model".to_string(), model.to_string());
        query_params.insert("year".to_string(), year.to_string());

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: Some(query_params),
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/device-definitions/{}", id);

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn list_device_makes(&self) -> Result<Value, Box<dyn Error>> {
        let path = "/device-makes".to_string();

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn get_device_type_by_id(&self, id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/device-types/{}", id);

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }
}
