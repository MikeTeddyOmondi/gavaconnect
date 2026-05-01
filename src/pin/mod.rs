use serde::{Deserialize, Serialize};

use crate::client::GavaConnectClient;
use crate::error::{GavaError, Result};

// ── Request types ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PinByIdRequest {
    #[serde(rename = "TaxpayerType")]
    pub taxpayer_type: String,
    #[serde(rename = "TaxpayerID")]
    pub taxpayer_id: String,
}

#[derive(Debug, Serialize)]
pub struct PinByPinRequest {
    #[serde(rename = "KRAPIN")]
    pub kra_pin: String,
}

#[derive(Debug, Serialize)]
pub struct PinGenerationDetails {
    #[serde(rename = "TaxpayerType")]
    pub taxpayer_type: String,
    #[serde(rename = "IdentificationNumber")]
    pub identification_number: String,
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: String,
    #[serde(rename = "MobileNumber")]
    pub mobile_number: String,
    #[serde(rename = "EmailAddress")]
    pub email_address: String,
    #[serde(rename = "IsPinWithNoOblig")]
    pub is_pin_with_no_oblig: String,
}

#[derive(Debug, Serialize)]
pub struct PinGenerationRequest {
    #[serde(rename = "TAXPAYERDETAILS")]
    pub taxpayer_details: PinGenerationDetails,
}

// ── Response types ──────────────────────────────────────────────────────────

/// Generic KRA JSON response — most endpoints return an envelope like this.
#[derive(Debug, Deserialize)]
pub struct KraResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl GavaConnectClient {
    /// Look up a KRA PIN using a national ID number.
    ///
    /// # Example
    /// ```no_run
    /// # use gavaconnect::{GavaConnectClient, pin::*};
    /// # #[tokio::main] async fn main() -> gavaconnect::Result<()> {
    /// let client = GavaConnectClient::sandbox("id", "secret");
    /// client.authenticate().await?;
    /// let resp = client.pin_checker_by_id("KE", "1000000").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pin_checker_by_id(
        &self,
        taxpayer_type: &str,
        taxpayer_id: &str,
    ) -> Result<serde_json::Value> {
        let body = PinByIdRequest {
            taxpayer_type: taxpayer_type.to_string(),
            taxpayer_id: taxpayer_id.to_string(),
        };
        let resp = self
            .post("/checker/v1/pin")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Look up taxpayer details using a KRA PIN string (e.g. `A123456789Z`).
    pub async fn pin_checker_by_pin(&self, kra_pin: &str) -> Result<serde_json::Value> {
        let body = PinByPinRequest {
            kra_pin: kra_pin.to_string(),
        };
        let resp = self
            .post("/checker/v1/pinbypin")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Generate a new KRA PIN for an individual.
    pub async fn generate_pin(&self, details: PinGenerationDetails) -> Result<serde_json::Value> {
        let body = PinGenerationRequest {
            taxpayer_details: details,
        };
        let resp = self
            .post("/v1/generate/pin")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Shared response handler — checks for HTTP and KRA-level errors.
    pub(crate) async fn handle_response(resp: reqwest::Response) -> Result<serde_json::Value> {
        let status = resp.status().as_u16();
        let body_text = resp.text().await.unwrap_or_default();

        if status == 401 {
            return Err(GavaError::Api {
                status,
                body: body_text,
            });
        }

        let value: serde_json::Value =
            serde_json::from_str(&body_text).map_err(|_| GavaError::Api {
                status,
                body: body_text.clone(),
            })?;

        // Check for KRA-specific error envelope.
        if let Some(code) = value.get("ErrorCode").and_then(|v| v.as_str()) {
            let message = value
                .get("ErrorMessage")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            return Err(GavaError::Kra {
                code: code.to_string(),
                message,
            });
        }

        if !(200..=299).contains(&status) {
            return Err(GavaError::Api {
                status,
                body: body_text,
            });
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_by_id_serialization() {
        let req = PinByIdRequest {
            taxpayer_type: "KE".into(),
            taxpayer_id: "1000000".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["TaxpayerType"], "KE");
        assert_eq!(json["TaxpayerID"], "1000000");
    }

    #[test]
    fn test_pin_by_pin_serialization() {
        let req = PinByPinRequest {
            kra_pin: "A123456789Z".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["KRAPIN"], "A123456789Z");
    }

    #[test]
    fn test_pin_generation_serialization() {
        let req = PinGenerationRequest {
            taxpayer_details: PinGenerationDetails {
                taxpayer_type: "KE".into(),
                identification_number: "12345678".into(),
                date_of_birth: "03/02/1990".into(),
                mobile_number: "0712345678".into(),
                email_address: "test@example.com".into(),
                is_pin_with_no_oblig: "No".into(),
            },
        };
        let json = serde_json::to_value(&req).unwrap();
        let details = &json["TAXPAYERDETAILS"];
        assert_eq!(details["TaxpayerType"], "KE");
        assert_eq!(details["DateOfBirth"], "03/02/1990");
        assert_eq!(details["IsPinWithNoOblig"], "No");
    }

    #[test]
    fn test_kra_error_detection() {
        let body = serde_json::json!({
            "RequestId": "abc-123",
            "ErrorCode": "84002",
            "ErrorMessage": "Inactive / Wrong PIN"
        });
        if let Some(code) = body.get("ErrorCode").and_then(|v| v.as_str()) {
            assert_eq!(code, "84002");
        } else {
            panic!("Should have detected error code");
        }
    }
}
