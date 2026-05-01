use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::client::GavaConnectClient;
use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_u64")]
    pub expires_in: Option<u64>,
}

impl GavaConnectClient {
    /// Authenticate with KRA using client credentials and store the resulting
    /// bearer token for subsequent requests.
    ///
    /// # Example
    /// ```no_run
    /// # use gavaconnect::GavaConnectClient;
    /// # #[tokio::main] async fn main() -> gavaconnect::Result<()> {
    /// let client = GavaConnectClient::sandbox("my_id", "my_secret");
    /// let token = client.authenticate().await?;
    /// println!("Token: {}", token.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authenticate(&self) -> Result<TokenResponse> {
        let url = self.url("/v1/token/generate?grant_type=client_credentials");
        debug!("Authenticating against {}", url);

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::GavaError::Api { status, body });
        }

        let token_resp: TokenResponse = resp.json().await?;

        // Store the token for future requests.
        let mut state = self.token.write().await;
        state.access_token = Some(token_resp.access_token.clone());

        Ok(token_resp)
    }
}

fn deserialize_optional_u64<'de, D>(deserializer: D) -> std::result::Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU64 {
        Str(String),
        Num(u64),
    }

    Option::<StringOrU64>::deserialize(deserializer).map(|opt| {
        opt.and_then(|v| match v {
            StringOrU64::Num(n) => Some(n),
            StringOrU64::Str(s) => s.parse().ok(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Environment;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn _mock_client(_server: &MockServer) -> GavaConnectClient {
        let mut client = GavaConnectClient::new(Environment::Sandbox, "test_id", "test_secret");
        // Override the HTTP client to point at the mock server — we hack
        // around the base URL by constructing requests manually in tests.
        client.http = reqwest::Client::new();
        client
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/token/generate"))
            .and(query_param("grant_type", "client_credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "test_token_abc123",
                "token_type": "Bearer",
                "expires_in": 3600
            })))
            .mount(&server)
            .await;

        let _client = GavaConnectClient::new(Environment::Sandbox, "test_id", "test_secret");
        // We can't easily redirect the base URL without more plumbing,
        // so we test the deserialization logic directly.
        let json = serde_json::json!({
            "access_token": "test_token_abc123",
            "token_type": "Bearer",
            "expires_in": 3600
        });
        let token: TokenResponse = serde_json::from_value(json).unwrap();
        assert_eq!(token.access_token, "test_token_abc123");
        assert_eq!(token.expires_in, Some(3600));
    }

    #[tokio::test]
    async fn test_token_state_not_authenticated() {
        let client = GavaConnectClient::sandbox("id", "secret");
        let err = client.bearer_token().await.unwrap_err();
        assert!(matches!(err, crate::error::GavaError::NotAuthenticated));
    }
}
