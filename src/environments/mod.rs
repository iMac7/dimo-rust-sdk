pub mod dimo_environment {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct Routes {
        pub attestation: &'static str,
        pub auth: &'static str,
        pub identity: &'static str,
        pub devices: &'static str,
        pub device_data: &'static str,
        pub device_definitions: &'static str,
        pub events: &'static str,
        pub telemetry: &'static str,
        pub token_exchange: &'static str,
        pub trips: &'static str,
        pub user: &'static str,
        pub valuations: &'static str,
        pub vehicle_signal_decoding: &'static str,
    }

    pub struct Constants {
        pub nft_address: &'static str,
        pub rpc_provider: &'static str,
        pub dlx_address: &'static str,
        pub vehicle_address: &'static str,
    }

    pub struct Environment {
        pub routes: Routes,
        pub constants: Constants,
    }

    pub const PRODUCTION: Environment = Environment {
        routes: Routes {
            attestation: "https://attestation-api.dimo.zone",
            auth: "https://auth.dimo.zone",
            identity: "https://identity-api.dimo.zone/query",
            devices: "https://devices-api.dimo.zone",
            device_data: "https://device-data-api.dimo.zone",
            device_definitions: "https://device-definitions-api.dimo.zone",
            events: "https://events-api.dimo.zone",
            telemetry: "https://telemetry-api.dimo.zone/query",
            token_exchange: "https://token-exchange-api.dimo.zone",
            trips: "https://trips-api.dimo.zone",
            user: "https://users-api.dimo.zone",
            valuations: "https://valuations-api.dimo.zone",
            vehicle_signal_decoding: "https://vehicle-signal-decoding.dimo.zone",
        },
        constants: Constants {
            nft_address: "0xbA5738a18d83D41847dfFbDC6101d37C69c9B0cF",
            rpc_provider: "https://eth.llamarpc.com",
            dlx_address: "0x9A9D2E717bB005B240094ba761Ff074d392C7C85",
            vehicle_address: "0xba5738a18d83d41847dffbdc6101d37c69c9b0cf",
        },
    };

    pub const DEV: Environment = Environment {
        routes: Routes {
            attestation: "https://attestation-api.dev.dimo.zone",
            auth: "https://auth.dev.dimo.zone",
            identity: "https://identity-api.dev.dimo.zone/query",
            devices: "https://devices-api.dev.dimo.zone",
            device_data: "https://device-data-api.dev.dimo.zone",
            device_definitions: "https://device-definitions-api.dev.dimo.zone",
            events: "https://events-api.dev.dimo.zone",
            telemetry: "https://telemetry-api.dev.dimo.zone/query",
            token_exchange: "https://token-exchange-api.dev.dimo.zone",
            trips: "https://trips-api.dev.dimo.zone",
            user: "https://users-api.dev.dimo.zone",
            valuations: "https://valuations-api.dev.dimo.zone",
            vehicle_signal_decoding: "https://vehicle-signal-decoding.dev.dimo.zone",
        },
        constants: Constants {
            nft_address: "0x45fbCD3ef7361d156e8b16F5538AE36DEdf61Da8",
            rpc_provider: "https://eth.llamarpc.com",
            dlx_address: "",
            vehicle_address: "0x45fbCD3ef7361d156e8b16F5538AE36DEdf61Da8",
        },
    };
}
