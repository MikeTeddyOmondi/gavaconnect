//! Basic authentication and PIN lookup.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example pin_lookup
//! ```

use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    dotenvy::dotenv().ok();

    let client_id = std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set");
    let client_secret =
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set");

    let client = GavaConnectClient::sandbox(&client_id, &client_secret);

    // Step 1: Authenticate
    let token = client.authenticate().await?;
    println!(
        "✓ Authenticated (expires in {:?}s)\n",
        token.expires_in.unwrap()
    );

    // Step 2: Look up a PIN by national ID
    let by_id = client.pin_checker_by_id("KE", "33503527").await?;
    println!("PIN by ID:\n{}\n", serde_json::to_string_pretty(&by_id)?);

    // Step 3: Look up by KRA PIN
    let by_pin = client.pin_checker_by_pin("A012007601Z").await?;
    println!("PIN by PIN:\n{}\n", serde_json::to_string_pretty(&by_pin)?);

    Ok(())
}
