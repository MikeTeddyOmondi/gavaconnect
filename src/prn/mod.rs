use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

/// The type of withholding tax PRN to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WithholdingType {
    /// Income Tax withholding (WHTIT).
    IncomeTax,
    /// Rental income withholding (WHTRENT).
    Rental,
    /// VAT withholding (WHTVAT).
    Vat,
}

impl WithholdingType {
    pub fn endpoint(&self) -> &'static str {
        match self {
            Self::IncomeTax => "/generate/v1/prn/whtit",
            Self::Rental => "/generate/v1/prn/whtrental",
            Self::Vat => "/generate/v1/prn/whtvat",
        }
    }

    pub fn obligation_code(&self) -> &'static str {
        match self {
            Self::IncomeTax => "WHTIT",
            Self::Rental => "WHTRENT",
            Self::Vat => "WHTVAT",
        }
    }
}

/// Common fields for a withholding tax PRN generation request.
/// The exact fields required may vary by endpoint — use `extra` for additional
/// key-value pairs that the KRA API expects.
#[derive(Debug, Serialize)]
pub struct PrnRequest {
    #[serde(rename = "withholderPin")]
    pub withholder_pin: String,
    #[serde(rename = "grossAmount")]
    pub gross_amount: f64,
    /// Any additional fields required by the specific PRN endpoint.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl GavaConnectClient {
    /// Generate a Payment Registration Number (PRN) for withholding tax.
    ///
    /// # Example
    /// ```no_run
    /// # use gavaconnect::{GavaConnectClient, prn::*};
    /// # #[tokio::main] async fn main() -> gavaconnect::Result<()> {
    /// let client = GavaConnectClient::sandbox("id", "secret");
    /// client.authenticate().await?;
    /// let req = PrnRequest {
    ///     withholder_pin: "A123456789Z".into(),
    ///     gross_amount: 100_000.0,
    ///     extra: serde_json::json!({}),
    /// };
    /// let resp = client.generate_prn(WithholdingType::IncomeTax, req).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_prn(
        &self,
        wht_type: WithholdingType,
        body: PrnRequest,
    ) -> Result<serde_json::Value> {
        let resp = self
            .post(wht_type.endpoint())
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
    fn test_withholding_type_endpoints() {
        assert_eq!(
            WithholdingType::IncomeTax.endpoint(),
            "/generate/v1/prn/whtit"
        );
        assert_eq!(
            WithholdingType::Rental.endpoint(),
            "/generate/v1/prn/whtrental"
        );
        assert_eq!(WithholdingType::Vat.endpoint(), "/generate/v1/prn/whtvat");
    }

    #[test]
    fn test_withholding_type_obligation_codes() {
        assert_eq!(WithholdingType::IncomeTax.obligation_code(), "WHTIT");
        assert_eq!(WithholdingType::Rental.obligation_code(), "WHTRENT");
        assert_eq!(WithholdingType::Vat.obligation_code(), "WHTVAT");
    }

    #[test]
    fn test_prn_request_serialization() {
        let req = PrnRequest {
            withholder_pin: "A123456789Z".into(),
            gross_amount: 250_000.0,
            extra: serde_json::json!({
                "payeePin": "B987654321X"
            }),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["withholderPin"], "A123456789Z");
        assert_eq!(json["grossAmount"], 250_000.0);
        assert_eq!(json["payeePin"], "B987654321X");
    }
}
