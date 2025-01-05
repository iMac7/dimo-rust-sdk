use std::env;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug)]
pub enum CredentialsError {
    MissingCredentials,
    MissingFields(Vec<&'static str>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Credentials {
    pub client_id: String,
    pub private_key: String,
    pub domain: String,
    pub developer_jwt: String,
    pub vehicle_jwt: String,
}

pub fn get_credentials() -> Result<Credentials, Box<dyn Error>> {
    let client_id = env::var("CLIENT_ID").unwrap_or_default();
    let private_key = env::var("API_KEY").unwrap_or_default();
    let domain = env::var("REDIRECT_URI").unwrap_or_default();
    let developer_jwt = env::var("DEVELOPER_JWT").ok().unwrap_or_default();
    let vehicle_jwt = env::var("VEHICLE_JWT").ok().unwrap_or_default();

    let mut missing_fields = Vec::new();
    if client_id.is_empty() {
        missing_fields.push("client_id");
    }
    if private_key.is_empty() {
        missing_fields.push("api_key");
    }
    if domain.is_empty() {
        missing_fields.push("redirect_uri");
    }

    if !missing_fields.is_empty() {
        return Err(format!("Missing required fields: {:?}", missing_fields).into());
    }

    Ok(Credentials {
        client_id,
        private_key,
        domain,
        developer_jwt,
        vehicle_jwt,
    })
}
