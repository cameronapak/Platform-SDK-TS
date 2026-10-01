use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchVersesClient {
    pub http_client: HttpClient,
}

impl SearchVersesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search for Bible verses by query, returning a paginated list of verse references. Verse results carry references and metadata only (e.g. JHN.3.16), never passage text; resolve verse text through the licensed-content endpoint. The bible_id determines the language searched, so this endpoint takes no language parameter. Use page_size and page_token to page through the full result set.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string used to find matching results.
    /// * `bible_id` - The Bible version identifier
    /// * `user_intent` - The searcher's intent. Defaults to unknown, matching the Core Search service.
    /// * `page_size` - The number of verse results to return in the collection. Must be between 1 and 99.
    /// * `page_token` - The page token to retrieve results from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1search_verses_collection_get(
        &self,
        request: &V1SearchVersesCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1SearchVersesCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/search-verses",
                None,
                QueryBuilder::new()
                    .string("query", request.query.clone())
                    .int("bible_id", request.bible_id.clone())
                    .serialize("user_intent", request.user_intent.clone())
                    .int("page_size", request.page_size.clone())
                    .string("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }
}
