use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct NilReturnDetails {
    #[serde(rename = "TaxpayerPIN")]
    pub taxpayer_pin: String,
    #[serde(rename = "ObligationCode")]
    pub obligation_code: String,
    #[serde(rename = "Month")]
    pub month: String,
    #[serde(rename = "Year")]
    pub year: String,
}

#[derive(Debug, Serialize)]
pub struct NilReturnRequest {
    #[serde(rename = "TAXPAYERDETAILS")]
    pub taxpayer_details: NilReturnDetails,
}

#[derive(Debug, Serialize)]
pub struct TotReturnDetails {
    #[serde(rename = "TaxpayerPIN")]
    pub taxpayer_pin: String,
    #[serde(rename = "Month")]
    pub month: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "GrossTurnover")]
    pub gross_turnover: u64,
}

#[derive(Debug, Serialize)]
pub struct TotReturnRequest {
    #[serde(rename = "TAXPAYERDETAILS")]
    pub taxpayer_details: TotReturnDetails,
}

impl GavaConnectClient {
    /// File a nil return for a given obligation period.
    pub async fn file_nil_return(&self, details: NilReturnDetails) -> Result<serde_json::Value> {
        let body = NilReturnRequest {
            taxpayer_details: details,
        };
        let resp = self
            .post("/dtd/return/v1/nil")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// File a Turnover Tax (TOT) return.
    pub async fn file_tot_return(&self, details: TotReturnDetails) -> Result<serde_json::Value> {
        let body = TotReturnRequest {
            taxpayer_details: details,
        };
        let resp = self
            .post("/filing/v1/tot/paymentregistration")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil_return_serialization() {
        let req = NilReturnRequest {
            taxpayer_details: NilReturnDetails {
                taxpayer_pin: "A123456789Z".into(),
                obligation_code: "IT".into(),
                month: "01".into(),
                year: "2025".into(),
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let details = &json["TAXPAYERDETAILS"];
        assert_eq!(details["TaxpayerPIN"], "A123456789Z");
        assert_eq!(details["ObligationCode"], "IT");
        assert_eq!(details["Month"], "01");
        assert_eq!(details["Year"], "2025");
    }

    #[test]
    fn test_tot_return_serialization() {
        let req = TotReturnRequest {
            taxpayer_details: TotReturnDetails {
                taxpayer_pin: "A123456789Z".into(),
                month: "07".into(),
                year: "2025".into(),
                gross_turnover: 50000,
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let details = &json["TAXPAYERDETAILS"];
        assert_eq!(details["GrossTurnover"], 50000);
    }
}
