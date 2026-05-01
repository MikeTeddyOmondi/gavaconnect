use serde::Serialize;

use crate::client::GavaConnectClient;
use crate::error::Result;

/// Headers required by all eTIMS OSCU endpoints.
#[derive(Debug, Clone)]
pub struct EtimsHeaders {
    pub tin: String,
    pub bhf_id: String,
    pub cmc_key: String,
    pub apigee_app_id: String,
}

/// A generic eTIMS request — most OSCU endpoints accept a JSON body with a
/// `lastReqDt` field plus operation-specific data.
#[derive(Debug, Serialize)]
pub struct EtimsRequest {
    #[serde(flatten)]
    pub body: serde_json::Value,
}

const ETIMS_BASE: &str = "/etims-oscu/api/v1";

impl GavaConnectClient {
    /// Internal helper to build an eTIMS POST with the required custom headers.
    async fn etims_post(
        &self,
        path: &str,
        headers: &EtimsHeaders,
    ) -> Result<reqwest::RequestBuilder> {
        let token = self.bearer_token().await?;
        let url = self.url(&format!("{}{}", ETIMS_BASE, path));
        Ok(self
            .http
            .post(url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .header("tin", &headers.tin)
            .header("bhfId", &headers.bhf_id)
            .header("cmcKey", &headers.cmc_key)
            .header("apigee_app_id", &headers.apigee_app_id))
    }

    // ── Branch Management ───────────────────────────────────────────────

    /// Fetch branch insurance info.
    pub async fn etims_branch_insurance_info(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/branchInsuranceInfo", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Manage branch user accounts.
    pub async fn etims_branch_user_account(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/branchUserAccount", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Send customer info to a branch.
    pub async fn etims_branch_send_customer_info(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/branchSendCustomerInfo", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ── Data Management ─────────────────────────────────────────────────

    /// Fetch the branch list.
    pub async fn etims_branch_list(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/branchList", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Fetch the code list (item classifications, tax types, etc.).
    pub async fn etims_select_code_list(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/selectCodeList", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Select taxpayer info.
    pub async fn etims_select_taxpayer_info(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/selectTaxpayerInfo", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Fetch customer PIN info.
    pub async fn etims_customer_pin_info(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/customerPinInfo", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ── Transactions ────────────────────────────────────────────────────

    /// Send a sales transaction.
    pub async fn etims_send_sales_transaction(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/sendSalesTransaction", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Send purchase transaction info.
    pub async fn etims_send_purchase_transaction(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/sendPurchaseTransactionInfo", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ── Stock Management ────────────────────────────────────────────────

    /// Insert a stock IO record.
    pub async fn etims_insert_stock_io(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/insert/stockIO", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Save a stock master record.
    pub async fn etims_save_stock_master(
        &self,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post("/save/stockMaster", headers)
            .await?
            .json(&body)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    /// Generic eTIMS endpoint caller — for any OSCU path not covered by a
    /// dedicated method.
    pub async fn etims_call(
        &self,
        path: &str,
        headers: &EtimsHeaders,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let resp = self
            .etims_post(path, headers)
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
    fn test_etims_headers() {
        let h = EtimsHeaders {
            tin: "P000111222R".into(),
            bhf_id: "00".into(),
            cmc_key: "key123".into(),
            apigee_app_id: "app456".into(),
        };
        assert_eq!(h.bhf_id, "00");
    }

    #[test]
    fn test_etims_request_serialization() {
        let body = serde_json::json!({
            "lastReqDt": "20250101000000",
            "tin": "P000111222R"
        });
        let req = EtimsRequest { body };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["lastReqDt"], "20250101000000");
    }

    #[test]
    fn test_etims_base_path() {
        assert_eq!(ETIMS_BASE, "/etims-oscu/api/v1");
    }
}
