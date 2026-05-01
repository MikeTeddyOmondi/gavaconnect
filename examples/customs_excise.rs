//! Customs declarations, import certificates, and excise license checks.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example customs_excise
//! ```

use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    // ── Customs ─────────────────────────────────────────────────────────

    // Declaration status
    match client.check_declaration("DEC-2025-001").await {
        Ok(data) => println!("Declaration:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Declaration error: {e}"),
    }

    // Import certificate by number
    match client.check_import_cert_by_number("IMP-123").await {
        Ok(data) => println!(
            "Import cert (num):\n{}\n",
            serde_json::to_string_pretty(&data)?
        ),
        Err(e) => eprintln!("Import cert error: {e}"),
    }

    // Import certificate by PIN
    match client.check_import_cert_by_pin("A123456789Z").await {
        Ok(data) => println!(
            "Import cert (PIN):\n{}\n",
            serde_json::to_string_pretty(&data)?
        ),
        Err(e) => eprintln!("Import cert error: {e}"),
    }

    // ── Excise ──────────────────────────────────────────────────────────

    match client.check_excise_by_pin("A123456789Z").await {
        Ok(data) => println!("Excise (PIN):\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Excise error: {e}"),
    }

    match client.check_excise_by_number("EXC-2025-001").await {
        Ok(data) => println!("Excise (num):\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Excise error: {e}"),
    }

    Ok(())
}
