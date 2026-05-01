//! Tax Compliance Certificate — apply and validate.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example tcc
//! ```

use gavaconnect::tcc::TccApplicationDetails;
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    let pin = "A123456789Z";

    // Apply for a TCC
    match client
        .apply_tcc(TccApplicationDetails {
            taxpayer_pin: pin.into(),
            reason: "Government tender".into(),
        })
        .await
    {
        Ok(data) => println!(
            "TCC application:\n{}\n",
            serde_json::to_string_pretty(&data)?
        ),
        Err(e) => eprintln!("TCC application error: {e}"),
    }

    // Validate an existing TCC
    match client.validate_tcc(pin, "TCC-99999").await {
        Ok(data) => println!(
            "TCC validation:\n{}\n",
            serde_json::to_string_pretty(&data)?
        ),
        Err(e) => eprintln!("TCC validation error: {e}"),
    }

    Ok(())
}
