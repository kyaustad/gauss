use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetadata {
    pub id: String,
    pub title: String,
    pub summary: Option<String>,
    pub release_date: Option<String>,
    pub default_cover_url: Option<String>,
    pub genres: Option<Vec<String>>,
    pub platforms: Option<Vec<String>>,
    pub source: MetadataSource,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MetadataSource {
    IGDB,
    Launchbox,
    RAWG,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub title: String,
    pub platform: Option<String>,
    pub year: Option<u16>,
}