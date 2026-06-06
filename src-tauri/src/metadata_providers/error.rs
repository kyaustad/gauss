
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("rate limited:")]
    RateLimited,

    #[error("Authorization Failed")]
    AuthFailure,

    #[error("provider error: {0}")]
    ProviderError(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
}
