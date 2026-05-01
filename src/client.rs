use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{GavaError, Result};

/// KRA environment selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Sandbox => "https://sbx.kra.go.ke",
            Environment::Production => "https://api.kra.go.ke",
        }
    }
}

/// Internal token state shared across clones.
#[derive(Debug, Default)]
pub(crate) struct TokenState {
    pub access_token: Option<String>,
}

/// The main SDK client. Clone-friendly (inner state is `Arc`-wrapped).
#[derive(Debug, Clone)]
pub struct GavaConnectClient {
    pub(crate) http: reqwest::Client,
    /// The active environment (Sandbox or Production).
    pub env: Environment,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) token: Arc<RwLock<TokenState>>,
}

impl GavaConnectClient {
    /// Create a new client targeting the given environment.
    pub fn new(
        env: Environment,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            env,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token: Arc::new(RwLock::new(TokenState::default())),
        }
    }

    /// Convenience constructor for the sandbox environment.
    pub fn sandbox(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::new(Environment::Sandbox, client_id, client_secret)
    }

    /// Convenience constructor for the production environment.
    pub fn production(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::new(Environment::Production, client_id, client_secret)
    }

    /// Full URL for a path segment, e.g. `/checker/v1/pin`.
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.env.base_url(), path)
    }

    /// Return the current bearer token or error if not authenticated.
    pub(crate) async fn bearer_token(&self) -> Result<String> {
        let state = self.token.read().await;
        state
            .access_token
            .clone()
            .ok_or(GavaError::NotAuthenticated)
    }

    /// Build an authenticated POST request.
    pub(crate) async fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let token = self.bearer_token().await?;
        Ok(self
            .http
            .post(self.url(path))
            .bearer_auth(token)
            .header("Content-Type", "application/json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_construction() {
        let client = GavaConnectClient::sandbox("id", "secret");
        assert_eq!(
            client.url("/checker/v1/pin"),
            "https://sbx.kra.go.ke/checker/v1/pin"
        );
        let prod = GavaConnectClient::production("id", "secret");
        assert_eq!(
            prod.url("/v1/token/generate"),
            "https://api.kra.go.ke/v1/token/generate"
        );
    }

    #[tokio::test]
    async fn test_bearer_token_not_authenticated() {
        let client = GavaConnectClient::sandbox("id", "secret");
        let err = client.bearer_token().await.unwrap_err();
        assert!(matches!(err, GavaError::NotAuthenticated));
    }

    #[tokio::test]
    async fn test_bearer_token_after_manual_set() {
        let client = GavaConnectClient::sandbox("id", "secret");
        {
            let mut state = client.token.write().await;
            state.access_token = Some("manual_token".to_string());
        }
        let token = client.bearer_token().await.unwrap();
        assert_eq!(token, "manual_token");
    }

    #[tokio::test]
    async fn test_post_not_authenticated() {
        let client = GavaConnectClient::sandbox("id", "secret");
        let err = client.post("/checker/v1/pin").await.unwrap_err();
        assert!(matches!(err, GavaError::NotAuthenticated));
    }
}
