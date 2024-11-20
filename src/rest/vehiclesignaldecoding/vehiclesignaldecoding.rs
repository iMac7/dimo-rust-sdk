use crate::utils::request::{make_auth_request, make_request, AuthRequestParams, RequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct VehicleSignalDecoding {
    base_url: String,
}

impl VehicleSignalDecoding {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn list_config_urls_by_vin(
        &self,
        vin: &str,
        protocol: Option<&str>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/vin/{}/urls", vin);
        let query_params = protocol.map(|protocol| {
            let mut q = HashMap::new();
            q.insert("protocol".to_string(), protocol.to_string());
            q
        });

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn list_config_urls_by_address(
        &self,
        address: &str,
        protocol: Option<&str>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/eth-addr/{}/urls", address);
        let query_params = protocol.map(|protocol| {
            let mut q = HashMap::new();
            q.insert("protocol".to_string(), protocol.to_string());
            q
        });

        let request_params = RequestParams {
            method: Method::GET,
            base_url: self.base_url.clone(),
            path,
            query_params,
            body: None,
            headers: None,
        };

        make_request(request_params).await
    }

    pub async fn get_pid_configs(&self, template_name: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/pids/{}", template_name);

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

    pub async fn get_device_settings(&self, template_name: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/settings/{}", template_name);

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

    pub async fn get_dbc_text(&self, template_name: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/dbc/{}", template_name);

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

    pub async fn get_device_status_by_address(
        &self,
        address: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/eth-addr/{}/status", address);

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

    pub async fn set_device_status_by_address(
        &self,
        address: &str,
        config: Value,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/eth-addr/{}/status", address);

        let mut body = HashMap::new();
        body.insert("config".to_string(), config);

        let request_params = AuthRequestParams {
            method: Method::PATCH,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: Some(body),
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_jobs_by_address(&self, address: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/eth-addr/{}/jobs", address);

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

    pub async fn get_pending_jobs_by_address(
        &self,
        address: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/device-config/eth-addr/{}/jobs/pending", address);

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

    pub async fn set_job_status_by_address(
        &self,
        address: &str,
        job_id: &str,
        status: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!(
            "/v1/device-config/eth-addr/{}/jobs/{}/{}",
            address, job_id, status
        );

        let request_params = AuthRequestParams {
            method: Method::PATCH,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        Ok(make_auth_request(request_params).await?)
    }
}
