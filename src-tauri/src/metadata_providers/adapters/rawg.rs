use async_trait::async_trait;

use crate::metadata_providers::deps::ProviderDeps;
use crate::metadata_providers::domain::{GameMetadata, MetadataSource, SearchQuery};
use crate::metadata_providers::error::MetadataError;
use crate::metadata_providers::port::MetadataProvider;

pub struct RawgProvider {
    deps: ProviderDeps,
    api_key: String,
}

impl RawgProvider {
    pub fn new(deps: ProviderDeps, api_key: String) -> Self {
        Self { deps, api_key }
    }
}

#[async_trait]
impl MetadataProvider for RawgProvider {
    fn source(&self) -> MetadataSource {
        MetadataSource::RAWG
    }

    async fn search(&self, _query: &SearchQuery) -> Result<Vec<GameMetadata>, MetadataError> {
        Err(MetadataError::ProviderError(
            "RAWG provider not implemented".into(),
        ))
    }

    async fn get_by_id(&self, _id: &str) -> Result<GameMetadata, MetadataError> {
        Err(MetadataError::ProviderError(
            "RAWG provider not implemented".into(),
        ))
    }
}
