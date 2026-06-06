use async_trait::async_trait;

use crate::metadata_providers::domain::{GameMetadata, MetadataSource, SearchQuery};
use crate::metadata_providers::error::MetadataError;

#[async_trait]
pub trait MetadataProvider: Send + Sync {
    fn source(&self) -> MetadataSource;

    async fn search(&self, query: &SearchQuery) -> Result<Vec<GameMetadata>, MetadataError>;

    async fn get_by_id(&self, id: &str) -> Result<GameMetadata, MetadataError>;
}
