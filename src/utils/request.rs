use crate::utils::get_credentials;
use reqwest::{Client, Method};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(serde::Serialize)]
pub enum QueryValue {
    Str(String),
    Bool(bool),
}

#[derive(Debug, serde::Serialize)]
pub enum BodyValue {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug)]
pub struct RequestParams {
    pub method: Method,
    pub base_url: String,
    pub path: String,
    pub query_params: Option<HashMap<String, String>>,
    pub body: Option<HashMap<String, Value>>,
    pub headers: Option<HashMap<String, String>>,
}

pub struct AuthRequestParams {
    pub method: Method,
    pub base_url: String,
    pub path: String,
    pub query_params: Option<HashMap<String, String>>,
    pub body: Option<HashMap<String, Value>>,
    pub headers: Option<HashMap<String, String>>,
    pub token_type: String,
}

/// Builds the authorization header using the appropriate token from `get_credentials`.
/// `token_type` should be either "developer" (for developer jwt) or "vehicle" (for vehicle jwt).
fn build_auth_header(token_type: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let credentials = get_credentials()?;

    let token = match token_type {
        "developer" => &credentials.developer_jwt,
        "vehicle" => &credentials.vehicle_jwt,
        _ => return Err("Invalid token type specified. Use 'developer' or 'vehicle'.".into()),
    };

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    Ok(headers)
}

pub async fn make_auth_request(params: AuthRequestParams) -> Result<Value, Box<dyn Error>> {
    let auth_header = build_auth_header(&params.token_type)?;
    let mut headers = params.headers.unwrap_or_default();
    headers.extend(auth_header);

    let request_params = RequestParams {
        method: params.method,
        base_url: params.base_url,
        path: params.path,
        query_params: params.query_params,
        body: params.body,
        headers: Some(headers),
    };

    make_request(request_params).await
}

pub async fn make_request(params: RequestParams) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}{}", params.base_url, params.path);

    let mut request_builder = match params.method {
        Method::GET => client.get(&url),
        Method::POST => client.post(&url),
        Method::PATCH => client.patch(&url),
        _ => return Err("Unsupported method".into()),
    };

    if let Some(ref query_params) = params.query_params {
        request_builder = request_builder.query(&query_params);
    };

    let use_form_body = params
        .headers
        .as_ref()
        .and_then(|headers| headers.get("Content-Type"))
        .map(|value| value.eq_ignore_ascii_case("x-www-form-urlencoded"))
        .unwrap_or(false);

    if let Some(body) = params.body {
        request_builder = if use_form_body {
            request_builder.form(&body)
        } else {
            request_builder.json(&body)
        };
    }

    if let Some(headers) = params.headers {
        for (key, value) in headers {
            request_builder = request_builder.header(&key, &value);
        }
    }

    let response_result = request_builder.send().await;

    match response_result {
        Ok(resp) => {
            if resp.status().is_success() {
                let json_response = resp.json::<Value>().await?;
                Ok(json_response)
            } else {
                let status = resp.status();
                let error_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(format!("Error {}: {}", status, error_text).into())
            }
        }
        Err(err) => Err(format!("Request error: {}", err).into()),
    }
}
