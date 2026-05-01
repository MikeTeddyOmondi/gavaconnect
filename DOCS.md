# gavaconnect — Developer Guide

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Getting Started](#getting-started)
3. [Authentication](#authentication)
4. [PIN Management](#pin-management)
5. [Checker Services](#checker-services)
6. [Customs & Import](#customs--import)
7. [Filing & Returns](#filing--returns)
8. [PRN Generation](#prn-generation)
9. [TCC (Tax Compliance Certificate)](#tcc)
10. [Excise License](#excise-license)
11. [eTIMS OSCU Integrator](#etims-oscu-integrator)
12. [Error Handling](#error-handling)
13. [Feature Flags](#feature-flags)
14. [Testing](#testing)

---

## Prerequisites

- Rust 1.75+ (2021 edition)
- A KRA developer account with **Client ID** and **Client Secret** (obtain from the KRA API portal)
- `tokio` runtime (the SDK is fully async)

## Getting Started

Add to your `Cargo.toml`:

```toml
[dependencies]
gavaconnect = "0.1"
tokio = { version = "1", features = ["full"] }
serde_json = "1"  # useful for inspecting raw responses
```

Create a client and authenticate:

```rust
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET");
    let token = client.authenticate().await?;
    println!("Authenticated. Token expires in {:?}s", token.expires_in);

    // All subsequent calls on `client` use the stored bearer token.
    Ok(())
}
```

> **Tip**: The token is stored internally. If it expires, call `authenticate()` again.

---

## Authentication

The `authenticate()` method hits KRA's `/v1/token/generate` endpoint with Basic Auth (your client credentials). It stores the resulting bearer token for all subsequent requests.

```rust
let token = client.authenticate().await?;
// token.access_token  — the raw JWT
// token.expires_in    — lifetime in seconds (Option<u64>)
```

---

## PIN Management

Requires the `pin` feature (enabled by default).

### Check PIN by National ID

```rust
let result = client.pin_checker_by_id("KE", "1000000").await?;
```

### Check PIN by KRA PIN

```rust
let result = client.pin_checker_by_pin("A123456789Z").await?;
```

### Generate Individual PIN

```rust
use gavaconnect::pin::PinGenerationDetails;

let details = PinGenerationDetails {
    taxpayer_type: "KE".into(),
    identification_number: "12345678".into(),
    date_of_birth: "03/02/1990".into(),   // DD/MM/YYYY
    mobile_number: "0712345678".into(),
    email_address: "dev@example.com".into(),
    is_pin_with_no_oblig: "No".into(),
};
let result = client.generate_pin(details).await?;
```

---

## Checker Services

Requires the `checker` feature.

### Know Your Station

```rust
let station = client.check_station("A123456789Z").await?;
// On success, look for station["RESPONSE"]["STATIONDATA"]["stationName"]
```

### Tax Obligations

```rust
let obligations = client.check_obligations("A123456789Z").await?;
```

### IT Exemption / VAT Exemption

```rust
let it = client.check_it_exemption("A123456789Z").await?;
let vat = client.check_vat_exemption("CERT-123").await?;
```

### Invoice Checker

```rust
let invoice = client.check_invoice("INV-001", "2025-06-15").await?;
```

### TCC Validation

```rust
let tcc = client.validate_tcc("A123456789Z", "TCC-99999").await?;
```

---

## Customs & Import

Requires the `customs` feature.

```rust
// Customs declaration status
let decl = client.check_declaration("DEC-2025-001").await?;

// Import certificate by number or by PIN
let cert = client.check_import_cert_by_number("IMP-123").await?;
let cert = client.check_import_cert_by_pin("A123456789Z").await?;
```

---

## Filing & Returns

Requires the `filing` feature.

### Nil Return

```rust
use gavaconnect::filing::NilReturnDetails;

let details = NilReturnDetails {
    taxpayer_pin: "A123456789Z".into(),
    obligation_code: "IT".into(),
    month: "01".into(),
    year: "2025".into(),
};
let result = client.file_nil_return(details).await?;
```

### Turnover Tax (TOT) Return

```rust
use gavaconnect::filing::TotReturnDetails;

let details = TotReturnDetails {
    taxpayer_pin: "A123456789Z".into(),
    month: "07".into(),
    year: "2025".into(),
    gross_turnover: 50_000,
};
let result = client.file_tot_return(details).await?;
```

---

## PRN Generation

Requires the `prn` feature. Generates Payment Registration Numbers for withholding taxes.

```rust
use gavaconnect::prn::{WithholdingType, PrnRequest};

let req = PrnRequest {
    withholder_pin: "A123456789Z".into(),
    gross_amount: 100_000.0,
    extra: serde_json::json!({
        "payeePin": "B987654321X"
    }),
};

// Income Tax WHT
let result = client.generate_prn(WithholdingType::IncomeTax, req).await?;

// Also available: WithholdingType::Rental, WithholdingType::Vat
```

---

## TCC

Requires the `tcc` feature.

### Apply for TCC

```rust
use gavaconnect::tcc::TccApplicationDetails;

let details = TccApplicationDetails {
    taxpayer_pin: "A123456789Z".into(),
    reason: "Government tender".into(),
};
let result = client.apply_tcc(details).await?;
```

### Validate TCC

TCC validation is in the `checker` module:

```rust
let result = client.validate_tcc("A123456789Z", "TCC-12345").await?;
```

---

## Excise License

Requires the `excise` feature.

```rust
let by_pin = client.check_excise_by_pin("A123456789Z").await?;
let by_num = client.check_excise_by_number("EXC-2025-001").await?;
```

---

## eTIMS OSCU Integrator

Requires the `etims` feature. This module covers the full eTIMS OSCU API surface.

### Setup Headers

Every eTIMS call requires four custom headers:

```rust
use gavaconnect::etims::EtimsHeaders;

let headers = EtimsHeaders {
    tin: "P000111222R".into(),
    bhf_id: "00".into(),
    cmc_key: "your_cmc_key".into(),
    apigee_app_id: "your_app_id".into(),
};
```

### Branch Management

```rust
let body = serde_json::json!({ "lastReqDt": "20250101000000" });

let branches = client.etims_branch_list(&headers, body.clone()).await?;
let insurance = client.etims_branch_insurance_info(&headers, body.clone()).await?;
let users = client.etims_branch_user_account(&headers, body.clone()).await?;
let customers = client.etims_branch_send_customer_info(&headers, body).await?;
```

### Data Management

```rust
let codes = client.etims_select_code_list(&headers, serde_json::json!({
    "lastReqDt": "20250101000000"
})).await?;

let taxpayer = client.etims_select_taxpayer_info(&headers, serde_json::json!({
    "tin": "P000111222R"
})).await?;

let customer = client.etims_customer_pin_info(&headers, serde_json::json!({
    "custTin": "A123456789Z"
})).await?;
```

### Sales & Purchase Transactions

```rust
let sale = client.etims_send_sales_transaction(&headers, serde_json::json!({
    "invcNo": 1,
    "rcptTyCd": "S",
    "pmtTyCd": "01",
    "salesSttsCd": "02",
    "itemList": []
})).await?;

let purchase = client.etims_send_purchase_transaction(&headers, serde_json::json!({
    "invcNo": 1,
    "spplrTin": "B987654321X",
    "itemList": []
})).await?;
```

### Stock Management

```rust
let stock_io = client.etims_insert_stock_io(&headers, serde_json::json!({
    "sarNo": 1,
    "itemList": []
})).await?;

let master = client.etims_save_stock_master(&headers, serde_json::json!({
    "itemCd": "ITEM001",
    "itemNm": "Widget"
})).await?;
```

### Generic eTIMS Call

For any OSCU endpoint not covered by a dedicated method:

```rust
let result = client.etims_call(
    "/someNewEndpoint",
    &headers,
    serde_json::json!({ "field": "value" }),
).await?;
```

---

## Error Handling

All methods return `gavaconnect::Result<serde_json::Value>`. The error type `GavaConnectError` covers:

| Variant                 | When                                               |
| ----------------------- | -------------------------------------------------- |
| `Http`                  | Network/transport failure                          |
| `Api { status, body }`  | Non-2xx HTTP status                                |
| `Deserialize`           | Response body isn't valid JSON                     |
| `NotAuthenticated`      | No token stored — call `authenticate()` first      |
| `Kra { code, message }` | KRA domain error (e.g. `84002 Inactive/Wrong PIN`) |

Pattern matching example:

```rust
use gavaconnect::GavaConnectError;

match client.check_station("BAD_PIN").await {
    Ok(v) => println!("{v}"),
    Err(GavaConnectError::Kra { code, message }) => {
        eprintln!("KRA says: [{code}] {message}");
    }
    Err(GavaConnectError::NotAuthenticated) => {
        client.authenticate().await?;
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

---

## Feature Flags

Disable `default-features` and pick only the modules you need to minimize compile time and binary size:

```toml
gavaconnect = { version = "0.1", default-features = false, features = ["pin", "filing"] }
```

Every module implicitly enables `auth` since all KRA API calls require a bearer token.

---

## Testing

Run the full test suite:

```bash
cargo test --all-features
```

Run tests for a specific module:

```bash
cargo test --features pin pin
cargo test --features etims etims
```

Unit tests cover serialization correctness for every request type. Integration tests validate client construction, URL generation, and token state management. To test against the real KRA sandbox, set `GAVACONNECT_CLIENT_ID` and `GAVACONNECT_CLIENT_SECRET` environment variables and run the integration suite (not included by default to avoid requiring credentials in CI).
