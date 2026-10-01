use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PermissionsClient {
    pub http_client: HttpClient,
}

impl PermissionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns permissions the authenticated user has already granted to the calling app. Use this
    /// endpoint before starting an OAuth permission request when you need to know whether the user
    /// has already granted access. The app ID path parameter must match the gateway-injected
    /// calling app ID. An empty permissions array means the user has not granted permissions for
    /// this app.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The unique identifier of the app.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1apps_permissions_collection_get(
        &self,
        app_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<V1AppsPermissionsCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/apps/{}/permissions",
                    crate::encode_path_segment(app_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }
}
