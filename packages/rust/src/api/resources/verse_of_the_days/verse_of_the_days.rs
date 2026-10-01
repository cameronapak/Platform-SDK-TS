use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct VerseOfTheDaysClient {
    pub http_client: HttpClient,
}

impl VerseOfTheDaysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// January 1 is day 1 and December 31 is day 365 (or 366 in a leap year).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1verse_of_the_days_canonical_collection_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Option<V1VerseOfTheDaysCanonicalCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/verse-of-the-days", None, None, options)
            .await
    }

    /// Day is the day of the year (1-366).
    ///
    /// # Arguments
    ///
    /// * `day` - The day of the year (1-366)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1verse_of_the_days_canonical_resource_get(
        &self,
        day: i64,
        options: Option<RequestOptions>,
    ) -> Result<V1VerseOfTheDaysCanonicalResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/verse-of-the-days/{}", day),
                None,
                None,
                options,
            )
            .await
    }
}
