# DIMO Rust SDK

For detailed API documentation, visit the [DIMO Developer Documentation](https://docs.dimo.org/developer-platform).

## Installation
Add the crate to your project's `cargo.toml` under `[dependencies]` . Ensure to check for the latest version.
```toml
[dependencies]
dimo-rust-sdk = "0.1.0"
```

## Using the SDK

To use the DIMO SDK in your Rust project, start by importing the SDK library:

```rust
use dimo_rust_sdk::{Environment, DIMO};
```

Then, initialize the SDK with the appropriate environment. The `Environment` enum supports two values: `Production` and `Dev`:

```rust
let mut dimo = DIMO::new(Environment::Production);
```

## Authentication

The SDK requires credentials, which should be passed via system environment variables. Below are the required and optional credentials:

#### Required Credentials:
- `client_id`
- `api_key` – The API key / private key for your DIMO account.
- `redirect_uri` – redirect uri / domain

#### Optional Credentials:
- `access_token` - Your Developer JWT
- `privilege_token` - Your Vehicle JWT

To set your credentials, export them as environment variables from your terminal:

```bash
export client_id="DIMO_CLIENT_ID"

export api_key="DIMO_API_KEY"

export redirect_uri="http://thatplace.com/doesntexist"
```

You can check the currently set credentials by calling the `get_credentials()` function:

```rust
use rust_sdk::{utils::request::get_credentials};

let credentials = get_credentials();
```

Alternatively, you can check the credentials directly via the terminal:

```bash
echo $client_id
```

### Permission Tokens

There are two types of tokens in the DIMO SDK:

1. **Developer JWT**: This token is generally used for authenticated endpoints.
    - To obtain the `Developer JWT`, ensure the three required environment variables (`client_id`, `api_key`, and `redirect_uri`) are set.
    ```rust
    let token = dimo.get_token().await;
   ```
    - Store the returned token string as an environment variable to use it.
   

2. **Vehicle JWT**: This token is needed for certain REST endpoints that require a `token_id`, and some GraphQL endpoints (`dimo.telemetry`).
    - Obtained by calling the token exchange endpoint. 
      ```rust
      dimo.tokenexchange.exchange(1, vec![2, 3])
      ```
    - The above example tries to get [privileges 2 and 3](https://docs.dimo.org/developer-platform/api-references/token-exchange-api/token-exchange-endpoints) for a vehicle of token id 1.

## Querying the REST API

To interact with the REST API, use the appropriate method in the SDK, passing the required parameters. Some methods will require a `Developer JWT` to authenticate the request.

All entry points in the main `dimo` struct are rest endpoints, except those in the graphql section below. 


```rust
dimo.devicedefinitions.get_by_id("0x23dfdf");
```

## Querying the GraphQL API

The SDK provides access to the GraphQL API through two entry points (`dimo.identity` , `dimo.telemetry`) in the `DIMO` struct, each with several methods available. 

`query()`: This method accepts any valid GraphQL query string and sends it to the respective endpoint.

```rust
let query = "
    {
        vehicles (first: 10) {
            totalCount
        }
    }
";

let result = dimo.identity.query(query).await;
```

This query is equivalent to calling `dimo.identity.count_dimo_vehicles()`.

> **Note**: The `telemetry` API (`dimo.telemetry`) requires a `Vehicle JWT`. Ensure that the appropriate token is set before querying telemetry-related endpoints.
