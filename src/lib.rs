pub mod environments;
pub mod graphql;
pub mod rest;
pub mod utils;

pub use serde_json::Value;
pub use utils::credentials::get_credentials;
use environments::dimo_environment;
use graphql::{Identity, Telemetry};
use rest::{
    attestation::AttestationClient,
    auth::{AccessToken, AuthClient},
    devicedefinitions::DeviceDefinitions,
    devices::Devices,
    tokenexchange::TokenExchange,
    trips::Trips,
    valuations::Valuations,
};

#[derive(Clone, Copy)]
pub enum Environment {
    Production,
    Dev,
}

pub struct DIMO {
    pub attestation: AttestationClient,
    pub auth: AuthClient,
    pub devicedefinitions: DeviceDefinitions,
    pub devices: Devices,
    pub tokenexchange: TokenExchange,
    pub trips: Trips,
    pub valuations: Valuations,
    pub identity: Identity,
    pub telemetry: Telemetry,
}

impl DIMO {
    pub fn new(env: Environment) -> Self {
        let routes = match env {
            Environment::Production => dimo_environment::PRODUCTION.routes,
            Environment::Dev => dimo_environment::DEV.routes,
        };
        let constants = match env {
            Environment::Production => dimo_environment::PRODUCTION.constants,
            Environment::Dev => dimo_environment::DEV.constants,
        };

        Self {
            attestation: AttestationClient::new(routes.attestation.to_string()),
            auth: AuthClient::new(routes.auth.to_string()),
            devicedefinitions: DeviceDefinitions::new(routes.device_definitions),
            devices: Devices::new(routes.devices),
            tokenexchange: TokenExchange::new(routes.token_exchange, constants.nft_address),
            trips: Trips::new(routes.trips),
            valuations: Valuations::new(routes.valuations),
            identity: Identity::new(routes.identity),
            telemetry: Telemetry::new(routes.telemetry),
        }
    }

    pub async fn get_token(&mut self) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let creds = get_credentials()?;

        let challenge = self
            .auth
            .generate_challenge(&creds.client_id, &creds.domain)
            .await
            .expect("error generating challenge");

        let state = challenge.state;
        let challenge = challenge.challenge;
        let signature = self
            .auth
            .sign_challenge(&challenge, &creds.private_key)
            .expect("error signing challenge");

        let token = self
            .auth
            .submit_challenge(&creds.client_id, &creds.domain, &state, &signature)
            .await
            .expect("error submitting challenge");

        Ok(token)
    }
}
