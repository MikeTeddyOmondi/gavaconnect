use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct DeclarationRequest {
    #[serde(rename = "DeclarationNo")]
    pub declaration_no: String,
}

#[derive(Debug, Serialize)]
pub struct ImportCertByNumRequest {
    #[serde(rename = "CertificateNo")]
    pub certificate_no: String,
}

#[derive(Debug, Serialize)]
pub struct ImportCertByPinRequest {
    #[serde(rename = "KRAPIN")]
    pub kra_pin: String,
}

impl GavaConnectClient {
    /// Check the status of a customs declaration.
    pub async fn check_declaration(&self, declaration_no: &str) -> Result<serde_json::Value> {
        let body = DeclarationRequest {
            declaration_no: declaration_no.to_string(),
        };
        let resp = self
            .post("/checker/v1/simple/declaration")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Validate an import certificate by certificate number.
    pub async fn check_import_cert_by_number(
        &self,
        certificate_no: &str,
    ) -> Result<serde_json::Value> {
        let body = ImportCertByNumRequest {
            certificate_no: certificate_no.to_string(),
        };
        let resp = self
            .post("/cbc/checker/v1/importcertificate/num")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Validate an import certificate by KRA PIN.
    pub async fn check_import_cert_by_pin(&self, kra_pin: &str) -> Result<serde_json::Value> {
        let body = ImportCertByPinRequest {
            kra_pin: kra_pin.to_string(),
        };
        let resp = self
            .post("/cbc/checker/v1/importcertificate/pin")
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
    fn test_declaration_serialization() {
        let req = DeclarationRequest {
            declaration_no: "DEC-2025-001".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["DeclarationNo"], "DEC-2025-001");
    }

    #[test]
    fn test_import_cert_by_num_serialization() {
        let req = ImportCertByNumRequest {
            certificate_no: "IMP-123".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["CertificateNo"], "IMP-123");
    }

    #[test]
    fn test_import_cert_by_pin_serialization() {
        let req = ImportCertByPinRequest {
            kra_pin: "A123456789Z".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["KRAPIN"], "A123456789Z");
    }
}
