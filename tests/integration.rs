use gavaconnect::{Environment, GavaConnectClient};

#[test]
fn test_sandbox_url() {
    assert_eq!(Environment::Sandbox.base_url(), "https://sbx.kra.go.ke");
}

#[test]
fn test_production_url() {
    assert_eq!(Environment::Production.base_url(), "https://api.kra.go.ke");
}

#[test]
fn test_client_constructors() {
    let sbx = GavaConnectClient::sandbox("id", "secret");
    assert_eq!(sbx.env, Environment::Sandbox);

    let prod = GavaConnectClient::production("id", "secret");
    assert_eq!(prod.env, Environment::Production);
}

#[test]
fn test_client_clone() {
    let client = GavaConnectClient::sandbox("id", "secret");
    let cloned = client.clone();
    assert_eq!(cloned.env, client.env);
}

// url() is pub(crate) — tested in unit tests.

// bearer_token() and post() are pub(crate) — tested in unit tests.

// Note: manual token injection tests live in src/auth/mod.rs (unit tests)
// since token state is pub(crate).

// ── Serialization round-trip tests for all modules ──────────────────────

#[cfg(feature = "pin")]
mod pin_tests {
    use gavaconnect::pin::*;

    #[test]
    fn pin_generation_round_trip() {
        let details = PinGenerationDetails {
            taxpayer_type: "KE".into(),
            identification_number: "12345678".into(),
            date_of_birth: "01/01/1990".into(),
            mobile_number: "0700000000".into(),
            email_address: "test@test.com".into(),
            is_pin_with_no_oblig: "No".into(),
        };
        let req = PinGenerationRequest {
            taxpayer_details: details,
        };
        let serialized = serde_json::to_string(&req).unwrap();
        assert!(serialized.contains("TAXPAYERDETAILS"));
        assert!(serialized.contains("IsPinWithNoOblig"));
    }
}

#[cfg(feature = "filing")]
mod filing_tests {
    use gavaconnect::filing::*;

    #[test]
    fn nil_return_round_trip() {
        let details = NilReturnDetails {
            taxpayer_pin: "A123456789Z".into(),
            obligation_code: "IT".into(),
            month: "12".into(),
            year: "2025".into(),
        };
        let req = NilReturnRequest {
            taxpayer_details: details,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["TAXPAYERDETAILS"]["Month"], "12");
    }

    #[test]
    fn tot_return_round_trip() {
        let details = TotReturnDetails {
            taxpayer_pin: "A123456789Z".into(),
            month: "06".into(),
            year: "2025".into(),
            gross_turnover: 1_000_000,
        };
        let req = TotReturnRequest {
            taxpayer_details: details,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["TAXPAYERDETAILS"]["GrossTurnover"], 1_000_000);
    }
}

#[cfg(feature = "prn")]
mod prn_tests {
    use gavaconnect::prn::*;

    #[test]
    fn prn_all_types() {
        let types = [
            WithholdingType::IncomeTax,
            WithholdingType::Rental,
            WithholdingType::Vat,
        ];
        for t in &types {
            assert!(!t.endpoint().is_empty());
            assert!(!t.obligation_code().is_empty());
        }
    }
}

#[cfg(feature = "etims")]
mod etims_tests {
    use gavaconnect::etims::*;

    #[test]
    fn etims_headers_construction() {
        let h = EtimsHeaders {
            tin: "P000111222R".into(),
            bhf_id: "00".into(),
            cmc_key: "abc".into(),
            apigee_app_id: "xyz".into(),
        };
        assert_eq!(h.tin, "P000111222R");
        assert_eq!(h.bhf_id, "00");
    }
}
