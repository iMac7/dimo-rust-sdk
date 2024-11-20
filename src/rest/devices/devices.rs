use crate::utils::request::{make_auth_request, make_request, AuthRequestParams, RequestParams};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct Devices {
    base_url: String,
}

impl Devices {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn create_vehicle(
        &self,
        country_code: &str,
        device_definition_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/v1/user/devices".to_string();
        let mut body = HashMap::new();
        body.insert(
            "countryCode".to_string(),
            Value::String(country_code.to_string()),
        );
        body.insert(
            "deviceDefinitionId".to_string(),
            Value::String(device_definition_id.to_string()),
        );

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

    pub async fn create_vehicle_from_smartcar(
        &self,
        code: &str,
        country_code: &str,
        redirect_uri: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/v1/user/devices/fromsmartcar".to_string();
        let mut body = HashMap::new();
        body.insert("code".to_string(), Value::String(code.to_string()));
        body.insert(
            "countryCode".to_string(),
            Value::String(country_code.to_string()),
        );
        body.insert(
            "redirectURI".to_string(),
            Value::String(redirect_uri.to_string()),
        );

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

    pub async fn create_vehicle_from_vin(
        &self,
        vin: &str,
        country_code: &str,
        can_protocol: Option<&str>,
    ) -> Result<Value, Box<dyn Error>> {
        let path = "/v1/user/devices/fromvin".to_string();
        let mut body = HashMap::new();
        body.insert("vin".to_string(), Value::String(vin.to_string()));
        body.insert(
            "countryCode".to_string(),
            Value::String(country_code.to_string()),
        );
        if let Some(protocol) = can_protocol {
            body.insert(
                "canProtocol".to_string(),
                Value::String(protocol.to_string()),
            );
        }

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

    pub async fn update_vehicle_vin(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/vin", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::PATCH,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_claiming_payload(&self, serial: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/aftermarket/device/by-serial/{}/commands/claim", serial);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn sign_claiming_payload(
        &self,
        serial: &str,
        claim_request: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/aftermarket/device/by-serial/{}/commands/claim", serial);
        let mut body = HashMap::new();
        body.insert(
            "claimRequest".to_string(),
            Value::String(claim_request.to_string()),
        );

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

    pub async fn get_minting_payload(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/commands/mint", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn sign_minting_payload(
        &self,
        user_device_id: &str,
        mint_request: Value,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/commands/mint", user_device_id);
        let mut body = HashMap::new();
        body.insert("mintRequest".to_string(), mint_request);

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

    pub async fn opt_in_share_data(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/commands/opt-in", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn refresh_smartcar_data(
        &self,
        user_device_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/commands/refresh", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_pairing_payload(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!(
            "/v1/user/devices/{}/aftermarket/commands/pair",
            user_device_id
        );

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

    pub async fn sign_pairing_payload(
        &self,
        user_device_id: &str,
        user_signature: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!(
            "/v1/user/devices/{}/aftermarket/commands/pair",
            user_device_id
        );
        let mut body = HashMap::new();
        body.insert(
            "userSignature".to_string(),
            Value::String(user_signature.to_string()),
        );

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

    pub async fn get_unpairing_payload(
        &self,
        user_device_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!(
            "/v1/user/devices/{}/aftermarket/commands/unpair",
            user_device_id
        );

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

    pub async fn sign_unpairing_payload(
        &self,
        user_device_id: &str,
        user_signature: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!(
            "/v1/user/devices/{}/aftermarket/commands/unpair",
            user_device_id
        );
        let mut body = HashMap::new();
        body.insert(
            "userSignature".to_string(),
            Value::String(user_signature.to_string()),
        );

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

    pub async fn lock_doors(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vehicle/{}/commands/doors/lock", token_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn unlock_doors(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vehicle/{}/commands/doors/unlock", token_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn open_frunk(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vehicle/{}/commands/frunk/open", token_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn open_trunk(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/vehicle/{}/commands/trunk/open", token_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "privilege".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn list_error_codes(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/error-codes", user_device_id);

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

    pub async fn submit_error_codes(
        &self,
        user_device_id: &str,
        query_device_error_codes: bool,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/error-codes", user_device_id);
        let mut body = HashMap::new();
        body.insert(
            "queryDeviceErrorCodes".to_string(),
            Value::Bool(query_device_error_codes),
        );

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

    pub async fn clear_error_codes(&self, user_device_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/user/devices/{}/error-codes/clear", user_device_id);

        let request_params = AuthRequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path,
            query_params: None,
            body: None,
            headers: None,
            token_type: "access".to_string(),
        };

        make_auth_request(request_params).await
    }

    pub async fn get_aftermarket_device(&self, token_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/aftermarket/device/{}", token_id);

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

    pub async fn get_aftermarket_device_image(
        &self,
        token_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/aftermarket/device/{}/image", token_id);

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

    pub async fn get_aftermarket_device_metadata_by_address(
        &self,
        address: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let path = format!("/v1/aftermarket/device/by-address/{}", address);

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
