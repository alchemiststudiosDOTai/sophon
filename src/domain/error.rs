#[derive(Debug, Clone, thiserror::Error)]
#[allow(dead_code)]
pub enum SearchError {
    #[error("invalid query: {0}")]
    InvalidQuery(String),
    #[error("provider configuration error: {0}")]
    Config(String),
    #[error("authentication failed")]
    Auth,
    #[error("rate limited")]
    RateLimited,
    #[error("transport error: {0}")]
    Transport(String),
    #[error("provider returned invalid data: {0}")]
    Decode(String),
    #[error("provider error: {0}")]
    Provider(String),
}
