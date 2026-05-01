//! Checker services — station lookup, obligations, exemptions, invoice validation.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example checker_services
//! ```

use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    let pin = "A123456789Z";

    // Know Your Station
    match client.check_station(pin).await {
        Ok(data) => println!("Station:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(gavaconnect::GavaConnectError::Kra { code, message }) => {
            eprintln!("KRA error [{code}]: {message}");
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    // Tax obligations
    match client.check_obligations(pin).await {
        Ok(data) => println!("Obligations:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Obligations error: {e}"),
    }

    // IT exemption
    match client.check_it_exemption(pin).await {
        Ok(data) => println!("IT Exemption:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("IT exemption error: {e}"),
    }

    // VAT exemption (by certificate number)
    match client.check_vat_exemption("CERT-123").await {
        Ok(data) => println!("VAT Exemption:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("VAT exemption error: {e}"),
    }

    // Invoice checker
    match client.check_invoice("INV-001", "2025-06-15").await {
        Ok(data) => println!("Invoice:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Invoice error: {e}"),
    }

    // TCC validation
    match client.validate_tcc(pin, "TCC-99999").await {
        Ok(data) => println!("TCC:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("TCC error: {e}"),
    }

    Ok(())
}
