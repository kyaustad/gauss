use reqwest::Client;

#[derive(Clone)]
pub struct ProviderDeps {
    pub http: Client,
}

#[derive(Clone)]
pub enum ProviderConfig {
    IGDB {
        client_id: String,
        client_secret: String,
    },
    Launchbox {
        data_path: String,
    },
    RAWG {
        api_key: String,
    },
}
