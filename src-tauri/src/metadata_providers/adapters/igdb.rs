use async_trait::async_trait;

use crate::metadata_providers::adapters::igdb_auth::IgdbAuth;
use crate::metadata_providers::deps::ProviderDeps;
use crate::metadata_providers::domain::{GameMetadata, MetadataSource, SearchQuery};
use crate::metadata_providers::error::MetadataError;
use crate::metadata_providers::port::MetadataProvider;

pub struct IGDBProvider {
    deps: ProviderDeps,
    auth: IgdbAuth,
    client_id: String,
}

impl IGDBProvider {
    pub fn new(deps: ProviderDeps, auth: IgdbAuth, client_id: String) -> Self {
        Self {
            deps,
            auth,
            client_id,
        }
    }
}

#[async_trait]
impl MetadataProvider for IGDBProvider {
    fn source(&self) -> MetadataSource {
        MetadataSource::IGDB
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<GameMetadata>, MetadataError> {
        let _token = self.auth.access_token().await?;
        let _ = query;
        todo!("search IGDB with access token")
    }

    async fn get_by_id(&self, id: &str) -> Result<GameMetadata, MetadataError> {
        let _token = self.auth.access_token().await?;
        let _ = id;
        todo!("fetch IGDB game by id")
    }
}
