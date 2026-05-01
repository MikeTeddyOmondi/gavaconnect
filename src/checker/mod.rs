use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

// ── Request types ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct StationRequest {
    #[serde(rename = "kraPIN")]
    pub kra_pin: String,
}

#[derive(Debug, Serialize)]
pub struct ObligationRequest {
    #[serde(rename = "taxPayerPin")]
    pub taxpayer_pin: String,
}

#[derive(Debug, Serialize)]
pub struct ItExemptionRequest {
    pub pin: String,
}

#[derive(Debug, Serialize)]
pub struct VatExemptionRequest {
    #[serde(rename = "VatExemptionCertificateNo")]
    pub certificate_no: String,
}

#[derive(Debug, Serialize)]
pub struct InvoiceRequest {
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: String,
    #[serde(rename = "invoiceDate")]
    pub invoice_date: String,
}

#[derive(Debug, Serialize)]
pub struct TccValidationRequest {
    #[serde(rename = "kraPIN")]
    pub kra_pin: String,
    #[serde(rename = "tccNumber")]
    pub tcc_number: String,
}

impl GavaConnectClient {
    /// Know Your Station — returns the KRA station assigned to a PIN.
    pub async fn check_station(&self, kra_pin: &str) -> Result<serde_json::Value> {
        let body = StationRequest {
            kra_pin: kra_pin.to_string(),
        };
        let resp = self
            .post("/dtd/checker/v1/station")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Check tax obligations registered against a KRA PIN.
    pub async fn check_obligations(&self, taxpayer_pin: &str) -> Result<serde_json::Value> {
        let body = ObligationRequest {
            taxpayer_pin: taxpayer_pin.to_string(),
        };
        let resp = self
            .post("/dtd/checker/v1/obligation")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Check Income Tax exemption status.
    pub async fn check_it_exemption(&self, pin: &str) -> Result<serde_json::Value> {
        let body = ItExemptionRequest {
            pin: pin.to_string(),
        };
        let resp = self
            .post("/checker/v1/itexemption")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Check VAT exemption certificate validity.
    pub async fn check_vat_exemption(&self, certificate_no: &str) -> Result<serde_json::Value> {
        let body = VatExemptionRequest {
            certificate_no: certificate_no.to_string(),
        };
        let resp = self
            .post("/dtd/checker/v1/vatexemption")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Validate an eTIMS invoice by number and date.
    pub async fn check_invoice(
        &self,
        invoice_number: &str,
        invoice_date: &str,
    ) -> Result<serde_json::Value> {
        let body = InvoiceRequest {
            invoice_number: invoice_number.to_string(),
            invoice_date: invoice_date.to_string(),
        };
        let resp = self
            .post("/checker/v1/invoice")
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Validate a Tax Compliance Certificate (TCC).
    pub async fn validate_tcc(&self, kra_pin: &str, tcc_number: &str) -> Result<serde_json::Value> {
        let body = TccValidationRequest {
            kra_pin: kra_pin.to_string(),
            tcc_number: tcc_number.to_string(),
        };
        let resp = self
            .post("/v1/kra-tcc/validate")
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
    fn test_station_request_serialization() {
        let req = StationRequest {
            kra_pin: "A744610021G".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["kraPIN"], "A744610021G");
    }

    #[test]
    fn test_obligation_request_serialization() {
        let req = ObligationRequest {
            taxpayer_pin: "A123456789Z".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["taxPayerPin"], "A123456789Z");
    }

    #[test]
    fn test_invoice_request_serialization() {
        let req = InvoiceRequest {
            invoice_number: "INV-001".into(),
            invoice_date: "2025-01-15".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["invoiceNumber"], "INV-001");
        assert_eq!(json["invoiceDate"], "2025-01-15");
    }

    #[test]
    fn test_tcc_validation_serialization() {
        let req = TccValidationRequest {
            kra_pin: "A123456789Z".into(),
            tcc_number: "TCC-12345".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["kraPIN"], "A123456789Z");
        assert_eq!(json["tccNumber"], "TCC-12345");
    }

    #[test]
    fn test_vat_exemption_serialization() {
        let req = VatExemptionRequest {
            certificate_no: "CERT-999".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["VatExemptionCertificateNo"], "CERT-999");
    }

    #[test]
    fn test_it_exemption_serialization() {
        let req = ItExemptionRequest {
            pin: "P001234567Q".into(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["pin"], "P001234567Q");
    }
}
