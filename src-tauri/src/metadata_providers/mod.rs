pub mod adapters;
pub mod deps;
pub mod domain;
pub mod error;
pub mod metadata_factory;
pub mod port;

pub use deps::{ProviderConfig, ProviderDeps};
pub use domain::{GameMetadata, MetadataSource, SearchQuery};
pub use error::MetadataError;
pub use metadata_factory::MetaFactory;
pub use port::MetadataProvider;
