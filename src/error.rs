use thiserror::Error;

/// All errors that can occur when interacting with the KRA API.
#[derive(Debug, Error)]
pub enum GavaConnectError {
    /// HTTP transport error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a non-success status code.
    #[error("API error {status}: {body}")]
    Api { status: u16, body: String },

    /// Failed to deserialize a response body.
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    /// The client has no valid access token — call `authenticate()` first.
    #[error("Not authenticated — call authenticate() before making API requests")]
    NotAuthenticated,

    /// KRA returned a domain-specific error code (e.g. 84002).
    #[error("KRA error {code}: {message}")]
    Kra { code: String, message: String },
}

pub type Result<T> = std::result::Result<T, GavaConnectError>;
