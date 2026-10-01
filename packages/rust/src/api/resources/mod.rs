//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **data exchange**
//! - **bibles**
//! - **highlights**
//! - **fonts**
//! - **SearchQueries**
//! - **SearchUnified**
//! - **SearchTopics**
//! - **SearchVerses**
//! - **languages**
//! - **licenses**
//! - **Apps**
//! - **permissions**
//! - **organizations**
//! - **verse of the days**

use crate::{ApiError, ClientConfig};

pub mod apps;
pub mod bibles;
pub mod data_exchange;
pub mod fonts;
pub mod highlights;
pub mod languages;
pub mod licenses;
pub mod organizations;
pub mod permissions;
pub mod search_queries;
pub mod search_topics;
pub mod search_unified;
pub mod search_verses;
pub mod verse_of_the_days;
pub struct PlatformClient {
    pub config: ClientConfig,
    pub data_exchange: DataExchangeClient,
    pub bibles: BiblesClient,
    pub highlights: HighlightsClient,
    pub fonts: FontsClient,
    pub search_queries: SearchQueriesClient,
    pub search_unified: SearchUnifiedClient,
    pub search_topics: SearchTopicsClient,
    pub search_verses: SearchVersesClient,
    pub languages: LanguagesClient,
    pub licenses: LicensesClient,
    pub apps: AppsClient,
    pub permissions: PermissionsClient,
    pub organizations: OrganizationsClient,
    pub verse_of_the_days: VerseOfTheDaysClient,
}

impl PlatformClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            data_exchange: DataExchangeClient::new(config.clone())?,
            bibles: BiblesClient::new(config.clone())?,
            highlights: HighlightsClient::new(config.clone())?,
            fonts: FontsClient::new(config.clone())?,
            search_queries: SearchQueriesClient::new(config.clone())?,
            search_unified: SearchUnifiedClient::new(config.clone())?,
            search_topics: SearchTopicsClient::new(config.clone())?,
            search_verses: SearchVersesClient::new(config.clone())?,
            languages: LanguagesClient::new(config.clone())?,
            licenses: LicensesClient::new(config.clone())?,
            apps: AppsClient::new(config.clone())?,
            permissions: PermissionsClient::new(config.clone())?,
            organizations: OrganizationsClient::new(config.clone())?,
            verse_of_the_days: VerseOfTheDaysClient::new(config.clone())?,
        })
    }
}

pub use apps::AppsClient;
pub use bibles::BiblesClient;
pub use data_exchange::DataExchangeClient;
pub use fonts::FontsClient;
pub use highlights::HighlightsClient;
pub use languages::LanguagesClient;
pub use licenses::LicensesClient;
pub use organizations::OrganizationsClient;
pub use permissions::PermissionsClient;
pub use search_queries::SearchQueriesClient;
pub use search_topics::SearchTopicsClient;
pub use search_unified::SearchUnifiedClient;
pub use search_verses::SearchVersesClient;
pub use verse_of_the_days::VerseOfTheDaysClient;
