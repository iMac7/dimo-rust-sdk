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
    pub access_token: String,
    pub privilege_token: String,
}

pub fn get_credentials() -> Result<Credentials, Box<dyn Error>> {
    let client_id = env::var("client_id").unwrap_or_default();
    let private_key = env::var("api_key").unwrap_or_default();
    let domain = env::var("redirect_uri").unwrap_or_default();
    let access_token = env::var("access_token").ok().unwrap_or_default();
    let privilege_token = env::var("privilege_token").ok().unwrap_or_default();

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
        access_token,
        privilege_token,
    })
}
