use std::sync::Arc;

use crate::metadata_providers::adapters::igdb::IGDBProvider;
use crate::metadata_providers::adapters::igdb_auth::IgdbAuth;
use crate::metadata_providers::adapters::launchbox::LaunchboxProvider;
use crate::metadata_providers::adapters::rawg::RawgProvider;
use crate::metadata_providers::deps::{ProviderConfig, ProviderDeps};
use crate::metadata_providers::error::MetadataError;
use crate::metadata_providers::port::MetadataProvider;

pub struct MetaFactory {
    deps: ProviderDeps,
    config: ProviderConfig,
}

impl MetaFactory {
    pub fn new(deps: ProviderDeps, config: ProviderConfig) -> Self {
        Self { deps, config }
    }

    pub fn create(&self) -> Result<Arc<dyn MetadataProvider>, MetadataError> {
        let provider: Arc<dyn MetadataProvider> = match &self.config {
            ProviderConfig::IGDB {
                client_id,
                client_secret,
            } => {
                let auth = IgdbAuth::new(
                    self.deps.clone(),
                    client_id.clone(),
                    client_secret.clone(),
                );
                Arc::new(IGDBProvider::new(
                    self.deps.clone(),
                    auth,
                    client_id.clone(),
                ))
            }
            ProviderConfig::Launchbox { data_path } => {
                Arc::new(LaunchboxProvider::new(self.deps.clone(), data_path.clone()))
            }
            ProviderConfig::RAWG { api_key } => {
                Arc::new(RawgProvider::new(self.deps.clone(), api_key.clone()))
            }
        };
        Ok(provider)
    }
}
