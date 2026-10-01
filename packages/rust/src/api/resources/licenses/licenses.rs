use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LicensesClient {
    pub http_client: HttpClient,
}

impl LicensesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of license metadata. By default, the response returns all licenses.
    /// In the case that `developer_id` is passed in the query, it filters to only include licenses
    /// the developer has agreed to. In the case that `bible_id` is passed it only returns the
    /// license under which that Bible ID is currently licensed. The license object also contains
    /// a list of all Bible ids are offered under that license.
    /// Optionally combine `developer_id` with the `all_available` query parameter to receive
    /// every license while still surfacing the developer's agreement metadata when present.
    ///
    /// # Arguments
    ///
    /// * `bible_id` - The Bible version identifier
    /// * `developer_id` - The Developer's unique ID in the Platform.
    /// * `all_available` - This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1licenses_collection_get(
        &self,
        request: &V1LicensesCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1LicensesCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/licenses",
                None,
                QueryBuilder::new()
                    .int("bible_id", request.bible_id.clone())
                    .string("developer_id", request.developer_id.clone())
                    .bool("all_available", request.all_available.clone())
                    .build(),
                options,
            )
            .await
    }
}
