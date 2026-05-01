//! eTIMS OSCU integrator — branch management, code lists, sales, stock.
//!
//! ```bash
//! GAVACONNECT_CLIENT_ID=xxx GAVACONNECT_CLIENT_SECRET=yyy \
//!   GAVACONNECT_ETIMS_TIN=P000111222R GAVACONNECT_ETIMS_BHF_ID=00 \
//!   GAVACONNECT_ETIMS_CMC_KEY=your_key GAVACONNECT_ETIMS_APP_ID=your_app \
//!   cargo run --example etims
//! ```

use gavaconnect::etims::EtimsHeaders;
use gavaconnect::GavaConnectClient;

fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} not set"))
}

#[tokio::main]
async fn main() -> gavaconnect::Result<()> {
    let client = GavaConnectClient::sandbox(
        env("GAVACONNECT_CLIENT_ID"),
        env("GAVACONNECT_CLIENT_SECRET"),
    );
    client.authenticate().await?;

    let headers = EtimsHeaders {
        tin: env("GAVACONNECT_ETIMS_TIN"),
        bhf_id: env("GAVACONNECT_ETIMS_BHF_ID"),
        cmc_key: env("GAVACONNECT_ETIMS_CMC_KEY"),
        apigee_app_id: env("GAVACONNECT_ETIMS_APP_ID"),
    };

    let last_req = serde_json::json!({ "lastReqDt": "20250101000000" });

    // ── Branch list ─────────────────────────────────────────────────────

    println!("--- Branch List ---");
    match client.etims_branch_list(&headers, last_req.clone()).await {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    // ── Code list ───────────────────────────────────────────────────────

    println!("--- Code List ---");
    match client
        .etims_select_code_list(&headers, last_req.clone())
        .await
    {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    // ── Taxpayer info ───────────────────────────────────────────────────

    println!("--- Taxpayer Info ---");
    match client
        .etims_select_taxpayer_info(&headers, serde_json::json!({ "tin": headers.tin }))
        .await
    {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    // ── Customer PIN info ───────────────────────────────────────────────

    println!("--- Customer PIN Info ---");
    match client
        .etims_customer_pin_info(&headers, serde_json::json!({ "custTin": "A123456789Z" }))
        .await
    {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    // ── Sales transaction ───────────────────────────────────────────────

    println!("--- Sales Transaction ---");
    let item = serde_json::json!({
        "itemSeq": 1,
        "itemCd": "ITEM001",
        "itemClsCd": "50101500",
        "itemNm": "Test Widget",
        "pkgUnitCd": "NT",
        "pkg": 1.0,
        "qtyUnitCd": "U",
        "qty": 1.0,
        "prc": 1000.0,
        "splyAmt": 1000.0,
        "dcRt": 0.0,
        "dcAmt": 0.0,
        "taxTyCd": "A",
        "taxblAmt": 1000.0,
        "taxAmt": 160.0,
        "totAmt": 1160.0
    });

    let sale_body = serde_json::json!({
        "invcNo": 1,
        "orgInvcNo": 0,
        "rcptTyCd": "S",
        "pmtTyCd": "01",
        "salesSttsCd": "02",
        "cfmDt": "20250715120000",
        "salesDt": "20250715",
        "stockRlsDt": null,
        "cnclReqDt": null,
        "cnclDt": null,
        "rfdDt": null,
        "totItemCnt": 1,
        "taxblAmtA": 1000.0,
        "taxblAmtB": 0.0,
        "taxblAmtC": 0.0,
        "taxblAmtD": 0.0,
        "taxRtA": 16.0,
        "taxRtB": 0.0,
        "taxRtC": 0.0,
        "taxRtD": 0.0,
        "taxAmtA": 160.0,
        "taxAmtB": 0.0,
        "taxAmtC": 0.0,
        "taxAmtD": 0.0,
        "totTaxblAmt": 1000.0,
        "totTaxAmt": 160.0,
        "totAmt": 1160.0,
        "remark": "SDK test sale",
        "itemList": [item]
    });

    match client
        .etims_send_sales_transaction(&headers, sale_body)
        .await
    {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    // ── Generic call (for endpoints without dedicated methods) ──────────

    println!("--- Generic eTIMS call ---");
    match client
        .etims_call("/branchInsuranceInfo", &headers, last_req)
        .await
    {
        Ok(data) => println!("{}\n", serde_json::to_string_pretty(&data)?),
        Err(e) => eprintln!("Error: {e}\n"),
    }

    Ok(())
}
