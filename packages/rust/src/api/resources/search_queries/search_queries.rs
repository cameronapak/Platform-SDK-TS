use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchQueriesClient {
    pub http_client: HttpClient,
}

impl SearchQueriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns query objects (a search string and its source). Supply query with language_ranges[] for as-you-type suggestions, or trending=true with language_ranges[] for recently popular searches. When trending=true, query is ignored. The first language range supported by search is used.
    ///
    /// # Arguments
    ///
    /// * `language_ranges_array` - An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    /// * `query` - The partial query for as-you-type suggestions. Omit for trending queries.
    /// * `trending` - Return recently popular searches for the language instead of suggestions.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1search_queries_collection_get(
        &self,
        request: &V1SearchQueriesCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<V1SearchQueriesCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/search-queries",
                None,
                QueryBuilder::new()
                    .string_array("language_ranges[]", request.language_ranges_array.clone())
                    .string("query", request.query.clone())
                    .bool("trending", request.trending.clone())
                    .build(),
                options,
            )
            .await
    }
}
