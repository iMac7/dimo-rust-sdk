use crate::utils::request::{make_request, RequestParams, make_auth_request, AuthRequestParams};
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

    /// Decodes a VIN (Vehicle Identification Number) and retrieves detailed information.
    /// Requires a developer jwt
    ///
    /// ### Body Parameters
    /// The `body` HashMap must include:
    /// - **`vin`** *(String, Required)*: Vehicle Identification Number.
    /// - **`countryCode`** *(String, Required)*: 3-letter ISO 3166-1 alpha-3 country code, e.g. "USA".
    ///
    /// ### Authentication
    /// Requires an access token to be passed (`access_token`) for authorization.
    ///
    /// ### Example
    /// ```ignore
    /// let mut body = HashMap::new();
    /// body.insert("vin".to_string(), "1HGCM82633A123456".to_string());
    /// body.insert("countryCode".to_string(), "US".to_string());
    ///
    /// let response = device_definitions.decode_vin(body, "access_token").await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn decode_vin(
        &self,
        body: HashMap<String, String>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/device-definitions/decode-vin".to_string();

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: Some(body.into_iter().map(|(k, v)| (k, Value::String(v))).collect()),
            headers: None,
            token_type: "developer".to_string(),
        };

        make_auth_request(request_params).await
    }

    /// Searches device definitions with a GraphQL query and optional additional parameters.
    ///
    /// ### Parameters
    /// - **`query`** *(String, Required)*: a valid GraphQL query string.
    /// - **`additional_params`** *(HashMap<String, String>, Optional)*:
    ///     - **`makeSlug`** *(String, Optional)*
    ///     - **`modelSlug`** *(String, Optional)*
    ///     - **`year`** *(String, Optional)*
    ///     - **`page`** *(String, Optional)*
    ///     - **`pageSize`** *(String, Optional)*
    ///
    /// ### Example
    /// ```ignore
    /// let query = "{ deviceDefinitions { id name } }";
    /// let mut additional_params = HashMap::new();
    /// additional_params.insert("makeSlug".to_string(), "toyota".to_string());
    /// additional_params.insert("year".to_string(), "2022".to_string());
    ///
    /// let response = device_definitions.search(query, Some(additional_params)).await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn search(
        &self,
        query: &str,
        additional_params: Option<HashMap<String, String>>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/device-definitions/search".to_string();
        let mut query_params = HashMap::new();
        query_params.insert("query".to_string(), query.to_string());

        if let Some(additional) = additional_params {
            query_params.extend(additional);
        }

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
}
