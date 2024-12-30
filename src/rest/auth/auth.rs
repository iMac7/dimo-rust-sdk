use crate::utils::request::{make_request, RequestParams};
use hex;
use reqwest::Method;
use secp256k1::{Message, Secp256k1, SecretKey};
use serde::Deserialize;
use serde_json::Value;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::error::Error;

pub struct AuthClient {
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeResponse {
    pub challenge: String,
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
    id_token: String,
    token_type: String,
    expires_in: i32,
}

impl AuthClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn generate_challenge(
        &self,
        client_id: &str,
        domain: &str,
    ) -> Result<ChallengeResponse, Box<dyn Error>> {
        let mut query_params: HashMap<String, String> = HashMap::new();
        query_params.insert("client_id".to_string(), client_id.to_string());
        query_params.insert("domain".to_string(), domain.to_string());
        query_params.insert("scope".to_string(), "openid email".to_string());
        query_params.insert("response_type".to_string(), "code".to_string());
        query_params.insert("address".to_string(), client_id.to_string());

        let params = RequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path: "/auth/web3/generate_challenge".to_string(),
            query_params: Some(query_params),
            body: None,
            headers: None,
        };

        let response = make_request(params).await?;
        let challenge_response: ChallengeResponse = serde_json::from_value(response)?;
        Ok(challenge_response)
    }

    pub fn sign_challenge(
        &self,
        message: &str,
        private_key: &str,
    ) -> Result<String, Box<dyn Error>> {
        let secp = Secp256k1::new();
        let private_key_bytes = hex::decode(private_key.strip_prefix("0x").unwrap_or(private_key))?;
        let secret_key = SecretKey::from_slice(&private_key_bytes)?;

        let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
        let prefixed_message = [prefix.as_bytes(), message.as_bytes()].concat();
        let digest = Keccak256::digest(&prefixed_message);
        let message = Message::from_digest_slice(&digest)?;
        let signature = secp.sign_ecdsa_recoverable(&message, &secret_key);

        let (recovery_id, signature_bytes) = signature.serialize_compact();
        let v = recovery_id.to_i32() as u8 + 27;
        let mut signature_full = Vec::with_capacity(65);
        signature_full.extend_from_slice(&signature_bytes);
        signature_full.push(v);

        Ok(hex::encode(signature_full))
    }

    pub async fn submit_challenge(
        &self,
        client_id: &str,
        domain: &str,
        state: &str,
        signature: &str,
    ) -> Result<AccessToken, Box<dyn Error>> {
        let mut body_params: HashMap<String, Value> = HashMap::new();
        body_params.insert(
            "client_id".to_string(),
            Value::String(client_id.to_string()),
        );
        body_params.insert("domain".to_string(), Value::String(domain.to_string()));
        body_params.insert(
            "grant_type".to_string(),
            Value::String("authorization_code".to_string()),
        );
        body_params.insert("state".to_string(), Value::String(state.to_string()));
        body_params.insert(
            "signature".to_string(),
            Value::String(format!("0x{}", signature.to_string())),
        );

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "x-www-form-urlencoded".to_string(),
        );

        let params = RequestParams {
            method: Method::POST,
            base_url: self.base_url.clone(),
            path: "/auth/web3/submit_challenge".to_string(),
            query_params: None,
            body: Some(body_params),
            headers: Some(headers),
        };

        let response = make_request(params).await.expect("could not submit");
        let token: AccessToken =
            serde_json::from_value(response).expect("could not parse response token");

        Ok(token)
    }
}
