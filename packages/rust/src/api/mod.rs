//! API client and types for the YouVersion Platform API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AppsClient, BiblesClient, DataExchangeClient, FontsClient, HighlightsClient, LanguagesClient,
    LicensesClient, OrganizationsClient, PermissionsClient, PlatformClient, SearchQueriesClient,
    SearchTopicsClient, SearchUnifiedClient, SearchVersesClient, VerseOfTheDaysClient,
};
pub use types::*;
