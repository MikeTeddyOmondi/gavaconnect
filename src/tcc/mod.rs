use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct TccApplicationDetails {
    #[serde(rename = "TaxpayerPIN")]
    pub taxpayer_pin: String,
    #[serde(rename = "ReasonForTCC")]
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct TccApplicationRequest {
    #[serde(rename = "TAXPAYERDETAILS")]
    pub taxpayer_details: TccApplicationDetails,
}

impl GavaConnectClient {
    /// Apply for a Tax Compliance Certificate.
    pub async fn apply_tcc(&self, details: TccApplicationDetails) -> Result<serde_json::Value> {
        let body = TccApplicationRequest {
            taxpayer_details: details,
        };
        let resp = self
            .post("/application/v1/tcc")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // Note: `validate_tcc` lives in the checker module since TCC validation
    // is listed under checker services in the KRA API.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcc_application_serialization() {
        let req = TccApplicationRequest {
            taxpayer_details: TccApplicationDetails {
                taxpayer_pin: "A123456789Z".into(),
                reason: "Government tender".into(),
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let details = &json["TAXPAYERDETAILS"];
        assert_eq!(details["TaxpayerPIN"], "A123456789Z");
        assert_eq!(details["ReasonForTCC"], "Government tender");
    }
}
