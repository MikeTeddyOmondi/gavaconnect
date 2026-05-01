# gavaconnect

A comprehensive Rust SDK for the **Kenya Revenue Authority (KRA) API**.

Covers the full surface of KRA's public API — PIN management, tax checkers, customs, return filing, PRN generation, TCC, excise licensing, and the eTIMS OSCU integrator — in a single, ergonomic crate with granular feature flags.

## Installation

```toml
[dependencies]
gavaconnect = "0.1"
tokio = { version = "1", features = ["full"] }
```

All modules are enabled by default. To pull in only what you need:

```toml
[dependencies]
gavaconnect = { version = "0.1", default-features = false, features = ["pin", "checker"] }
```

## Quick Start

```rust
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox("CLIENT_ID", "CLIENT_SECRET");
    client.authenticate().await?;

    // Check a KRA station
    let station = client.check_station("A123456789Z").await?;
    println!("{:#?}", station);

    // Look up a PIN
    let pin_info = client.pin_checker_by_pin("A123456789Z").await?;
    println!("{:#?}", pin_info);

    Ok(())
}
```

## Modules & Features

| Feature   | Module    | APIs Covered                                            |
| --------- | --------- | ------------------------------------------------------- |
| `auth`    | `auth`    | OAuth2 token generation (client credentials)            |
| `pin`     | `pin`     | PIN checker by ID, by PIN, individual PIN generation    |
| `checker` | `checker` | Station, obligations, IT/VAT exemption, invoice, TCC    |
| `customs` | `customs` | Customs declaration status, import certificate validity |
| `filing`  | `filing`  | Nil return filing, TOT return filing                    |
| `prn`     | `prn`     | Withholding tax PRN generation (IT, Rental, VAT)        |
| `tcc`     | `tcc`     | Tax Compliance Certificate application                  |
| `excise`  | `excise`  | Excise license checker (by PIN, by number)              |
| `etims`   | `etims`   | eTIMS OSCU integrator — branch, data, sales, stock      |

The `full` feature (default) activates everything. Each module depends on `auth`.

## Environments

```rust
// Sandbox (development/testing)
let client = GavaConnectClient::sandbox("id", "secret");

// Production
let client = GavaConnectClient::production("id", "secret");

// Explicit
use gavaconnect::Environment;
let client = GavaConnectClient::new(Environment::Sandbox, "id", "secret");
```

## Error Handling

All methods return `gavaconnect::Result<T>`, which wraps `GavaConnectError`:

```rust
match client.check_station("A123456789Z").await {
    Ok(data) => println!("Station: {}", data),
    Err(gavaconnect::GavaConnectError::NotAuthenticated) => {
        client.authenticate().await?;
        // retry...
    }
    Err(gavaconnect::GavaConnectError::Kra { code, message }) => {
        eprintln!("KRA error {}: {}", code, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
