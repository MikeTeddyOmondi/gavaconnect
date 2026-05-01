//! Individual PIN generation.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example pin_generation
//! ```

use gavaconnect::pin::PinGenerationDetails;
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    let result = client
        .generate_pin(PinGenerationDetails {
            taxpayer_type: "KE".into(),
            identification_number: "12345678".into(),
            date_of_birth: "03/02/1990".into(),
            mobile_number: "0712345678".into(),
            email_address: "dev@example.com".into(),
            is_pin_with_no_oblig: "No".into(),
        })
        .await;

    match result {
        Ok(data) => println!("PIN generated:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("PIN generation error: {e}"),
    }

    Ok(())
}
