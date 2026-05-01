use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct ExciseByPinRequest {
    #[serde(rename = "PINNo")]
    pub pin_no: String,
}

#[derive(Debug, Serialize)]
pub struct ExciseByNumberRequest {
    #[serde(rename = "ExciseLicenceNo")]
    pub excise_licence_no: String,
}

impl GavaConnectClient {
    /// Check an excise license by KRA PIN.
    pub async fn check_excise_by_pin(&self, pin: &str) -> Result<serde_json::Value> {
        let body = ExciseByPinRequest {
            pin_no: pin.to_string(),
        };
        let resp = self
            .post("/checker/v1/ExciseLicenseByPin")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Check an excise license by license number.
    pub async fn check_excise_by_number(&self, licence_no: &str) -> Result<serde_json::Value> {
        let body = ExciseByNumberRequest {
            excise_licence_no: licence_no.to_string(),
        };
        let resp = self
            .post("/checker/v1/ExciseLicenseByNum")
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
    fn test_excise_by_pin_serialization() {
        let req = ExciseByPinRequest {
            pin_no: "A123456789Z".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["PINNo"], "A123456789Z");
    }

    #[test]
    fn test_excise_by_number_serialization() {
        let req = ExciseByNumberRequest {
            excise_licence_no: "EXC-2025-001".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["ExciseLicenceNo"], "EXC-2025-001");
    }
}
