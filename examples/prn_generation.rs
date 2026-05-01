//! Withholding tax PRN generation — IT, Rental, and VAT.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example prn_generation
//! ```

use gavaconnect::prn::{PrnRequest, WithholdingType};
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    let types = [
        ("Income Tax", WithholdingType::IncomeTax),
        ("Rental", WithholdingType::Rental),
        ("VAT", WithholdingType::Vat),
    ];

    for (label, wht_type) in &types {
        let req = PrnRequest {
            withholder_pin: "A123456789Z".into(),
            gross_amount: 100_000.0,
            extra: serde_json::json!({
                "payeePin": "B987654321X"
            }),
        };

        match client.generate_prn(*wht_type, req).await {
            Ok(data) => println!("{label} PRN:\n{}\n", serde_json::to_string_pretty(&data)?),
            Err(e) => eprintln!("{label} PRN error: {e}"),
        }
    }

    Ok(())
}
