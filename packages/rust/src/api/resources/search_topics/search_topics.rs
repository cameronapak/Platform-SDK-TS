use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchTopicsClient {
    pub http_client: HttpClient,
}

impl SearchTopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the set of topics (id, text, subtopics) related to a query, for pivoting to other verses in the same topic.
    ///
    /// This endpoint is unpaginated: Core Search returns a fixed set of topics and exposes no page_size or page_token parameters. Query metadata (did_you_mean, search_instead_for) is always included.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string used to find matching results.
    /// * `language_ranges_array` - An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1search_topics_collection_get(
        &self,
        request: &V1SearchTopicsCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1SearchTopicsCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/search-topics",
                None,
                QueryBuilder::new()
                    .string("query", request.query.clone())
                    .string_array("language_ranges[]", request.language_ranges_array.clone())
                    .build(),
                options,
            )
            .await
    }
}
