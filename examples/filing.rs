//! Filing returns — nil return and Turnover Tax (TOT).
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy cargo run --example filing
//! ```

use gavaconnect::filing::{NilReturnDetails, TotReturnDetails};
use gavaconnect::GavaConnectClient;

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        std::env::var("GAVACONNECT_CLIENT_ID").expect("GAVACONNECT_CLIENT_ID not set"),
        std::env::var("GAVACONNECT_CLIENT_SECRET").expect("GAVACONNECT_CLIENT_SECRET not set"),
    );
    client.authenticate().await?;

    let pin = "A123456789Z";

    // File a nil return
    let nil = client
        .file_nil_return(NilReturnDetails {
            taxpayer_pin: pin.into(),
            obligation_code: "IT".into(),
            month: "01".into(),
            year: "2025".into(),
        })
        .await;

    match nil {
        Ok(data) => println!("Nil return:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Nil return error: {e}"),
    }

    // File a TOT return
    let tot = client
        .file_tot_return(TotReturnDetails {
            taxpayer_pin: pin.into(),
            month: "07".into(),
            year: "2025".into(),
            gross_turnover: 50_000,
        })
        .await;

    match tot {
        Ok(data) => println!("TOT return:\n{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("TOT return error: {e}"),
    }

    Ok(())
}
