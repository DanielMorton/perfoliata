use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("Missing field: {0}")]
    MissingField(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for ClientError {
    fn from(msg: String) -> Self {
        ClientError::Other(msg)
    }
}

impl From<&str> for ClientError {
    fn from(msg: &str) -> Self {
        ClientError::Other(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;
