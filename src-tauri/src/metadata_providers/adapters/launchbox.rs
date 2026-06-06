use async_trait::async_trait;

use crate::metadata_providers::deps::ProviderDeps;
use crate::metadata_providers::domain::{GameMetadata, MetadataSource, SearchQuery};
use crate::metadata_providers::error::MetadataError;
use crate::metadata_providers::port::MetadataProvider;

pub struct LaunchboxProvider {
    deps: ProviderDeps,
    data_path: String,
}

impl LaunchboxProvider {
    pub fn new(deps: ProviderDeps, data_path: String) -> Self {
        Self { deps, data_path }
    }
}

#[async_trait]
impl MetadataProvider for LaunchboxProvider {
    fn source(&self) -> MetadataSource {
        MetadataSource::Launchbox
    }

    async fn search(&self, _query: &SearchQuery) -> Result<Vec<GameMetadata>, MetadataError> {
        Err(MetadataError::ProviderError(
            "Launchbox provider not implemented".into(),
        ))
    }

    async fn get_by_id(&self, _id: &str) -> Result<GameMetadata, MetadataError> {
        Err(MetadataError::ProviderError(
            "Launchbox provider not implemented".into(),
        ))
    }
}
