use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AppsClient {
    pub http_client: HttpClient,
}

impl AppsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a single app resource by its id.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The unique identifier of the app.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1apps_resource_get(
        &self,
        app_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<V1AppsResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/apps/{}", crate::encode_path_segment(app_id)?),
                None,
                None,
                options,
            )
            .await
    }
}
