use crate::metadata_providers::deps::ProviderDeps;
use crate::metadata_providers::error::MetadataError;

#[derive(Clone)]
pub struct IgdbAuth {
    deps: ProviderDeps,
    client_id: String,
    client_secret: String,
}

impl IgdbAuth {
    pub fn new(deps: ProviderDeps, client_id: String, client_secret: String) -> Self {
        Self {
            deps,
            client_id,
            client_secret,
        }
    }

    pub async fn access_token(&self) -> Result<String, MetadataError> {
        todo!("fetch Twitch OAuth token using client_id and client_secret")
    }
}
