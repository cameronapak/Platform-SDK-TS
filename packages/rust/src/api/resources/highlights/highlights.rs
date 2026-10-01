use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct HighlightsClient {
    pub http_client: HttpClient,
}

impl HighlightsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The response will return a color per verse without ranges.
    ///
    /// # Arguments
    ///
    /// * `bible_id` - The Bible version identifier
    /// * `passage_id` - The passage identifier (verse or chapter USFM format)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1highlights_collection_get(
        &self,
        request: &V1HighlightsCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<V1HighlightsCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/highlights",
                None,
                QueryBuilder::new()
                    .int("bible_id", request.bible_id.clone())
                    .string("passage_id", request.passage_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Verse ranges may be used in the POST body passage_id attribute.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1highlights_collection_post(
        &self,
        request: &V1HighlightsCollectionPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1HighlightsCollectionPostResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/highlights",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Clear highlights for a passage.
    ///
    /// # Arguments
    ///
    /// * `passage_id_path` - The passage identifier (verse or chapter USFM format)
    /// * `bible_id` - The Bible version identifier
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn v1highlights_resource_delete(
        &self,
        passage_id_path: &str,
        request: &V1HighlightsResourceDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/highlights/{}",
                    crate::encode_path_segment(passage_id_path)?
                ),
                None,
                QueryBuilder::new()
                    .int("bible_id", request.bible_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
