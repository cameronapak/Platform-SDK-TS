use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchUnifiedClient {
    pub http_client: HttpClient,
}

impl SearchUnifiedClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search for Bible verses and related topics by query, returning a single unified set of results across all content kinds. Verse results carry references and metadata only (e.g. JHN.3.16), never passage text; resolve verse text through the licensed-content endpoint.
    ///
    /// This endpoint is unpaginated: it returns one combined page of top results per kind and exposes no page_size or page_token parameters. Use the kind-specific endpoints (search-verses, search-topics) when you need to page through the full result set for a single kind.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string used to find matching results.
    /// * `bible_id` - The Bible version identifier
    /// * `language_ranges_array` - An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    /// * `user_intent` - The searcher's intent. Defaults to unknown, matching the Core Search service.
    /// * `fields_array` - Result kinds to include: `verses` and/or `topics`. Use bracket notation to pass multiple
    /// values, for example `fields[]=verses&fields[]=topics`. Omit to return all kinds. Query
    /// metadata is always included.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1search_unified_collection_get(
        &self,
        request: &V1SearchUnifiedCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1SearchUnifiedCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/search-unified",
                None,
                QueryBuilder::new()
                    .string("query", request.query.clone())
                    .int("bible_id", request.bible_id.clone())
                    .string_array("language_ranges[]", request.language_ranges_array.clone())
                    .serialize("user_intent", request.user_intent.clone())
                    .string_array("fields[]", request.fields_array.clone())
                    .build(),
                options,
            )
            .await
    }
}
