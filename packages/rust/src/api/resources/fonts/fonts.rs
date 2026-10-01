use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FontsClient {
    pub http_client: HttpClient,
}

impl FontsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a collection of font family resources with their available variants and CDN sources.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1fonts_collection_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<V1FontsCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/fonts", None, None, options)
            .await
    }

    /// Get a single font family resource by its integer identifier.
    ///
    /// # Arguments
    ///
    /// * `font_id` - The unique integer identifier for a font family resource in the Platform.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1fonts_resource_get(
        &self,
        font_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<V1FontsResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/fonts/{}", font_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a text/css stylesheet for a single font family resource using the canonical woff2 CDN URLs exposed by the JSON Fonts API for browser consumption. The API gateway accepts `app_key` on this route and injects the required app-id header before forwarding the request to this service.
    ///
    /// # Arguments
    ///
    /// * `font_id` - The unique integer identifier for a font family resource in the Platform.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
    pub async fn v1fonts_stylesheet_get(
        &self,
        font_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/fonts/{}/stylesheet", font_id),
                None,
                None,
                options,
            )
            .await
    }
}
