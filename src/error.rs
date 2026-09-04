use thiserror::Error;

#[derive(Error, Debug)]
pub enum InnertubeError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Protobuf encode/decode error: {0}")]
    Proto(#[from] prost::DecodeError),

    #[error("Player decipher failed: {0}")]
    Player(String),

    #[error("API error ({status}): {message}")]
    Api { status: String, message: String },

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Content is restricted: {0}")]
    Restricted(String),

    #[error("Authentication is required: {0}")]
    AuthenticationRequired(String),

    #[error("OAuth2 error: {0}")]
    OAuth2(String),

    #[error("Format error: {0}")]
    Format(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, InnertubeError>;
