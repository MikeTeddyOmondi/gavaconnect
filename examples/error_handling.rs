//! Error handling patterns — token refresh, KRA errors, retries.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example error_handling
//! ```

use gavaconnect::{GavaConnectClient, GavaConnectError};

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );

    // ── Pattern 1: Call without authenticating first ────────────────────

    println!("--- Calling without auth ---");
    match client.check_station("A123456789Z").await {
        Err(GavaConnectError::NotAuthenticated) => {
            println!("Not authenticated — acquiring token...");
            client.authenticate().await?;
            println!("✓ Token acquired\n");
        }
        other => println!("Unexpected: {other:?}\n"),
    }

    // ── Pattern 2: Handle KRA domain errors ────────────────────────────

    println!("--- KRA domain error ---");
    match client.check_station("INVALID_PIN").await {
        Ok(data) => println!("Success: {data}"),
        Err(GavaConnectError::Kra { code, message }) => {
            println!("KRA rejected the request:");
            println!("  Code:    {code}");
            println!("  Message: {message}\n");
        }
        Err(e) => eprintln!("Other error: {e}\n"),
    }

    // ── Pattern 3: Retry with re-auth on 401 ───────────────────────────

    println!("--- Retry on 401 ---");
    let result = call_with_retry(&client, "A123456789Z").await;
    match result {
        Ok(data) => println!("Final result:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Failed after retry: {e}\n"),
    }

    // ── Pattern 4: Exhaustive match ────────────────────────────────────

    println!("--- Exhaustive match ---");
    match client.check_obligations("A123456789Z").await {
        Ok(data) => println!("Obligations: {data}"),
        Err(GavaConnectError::NotAuthenticated) => eprintln!("Need to authenticate"),
        Err(GavaConnectError::Http(e)) => eprintln!("Network problem: {e}"),
        Err(GavaConnectError::Api { status, body }) => eprintln!("HTTP {status}: {body}"),
        Err(GavaConnectError::Deserialize(e)) => eprintln!("Bad response format: {e}"),
        Err(GavaConnectError::Kra { code, message }) => eprintln!("KRA [{code}]: {message}"),
    }

    Ok(())
}

/// Attempt a call, re-authenticate once on 401, then retry.
async fn call_with_retry(
    client: &GavaConnectClient,
    pin: &str,
) -> gavaconnect::Result<serde_json::Value> {
    match client.check_station(pin).await {
        Ok(data) => Ok(data),
        Err(GavaConnectError::Api { status: 401, .. }) => {
            println!("  → 401 received, refreshing token...");
            client.authenticate().await?;
            client.check_station(pin).await
        }
        Err(e) => Err(e),
    }
}
