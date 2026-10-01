use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OrganizationsClient {
    pub http_client: HttpClient,
}

impl OrganizationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of Organization objects. Use bible_ids[] to filter
    /// to organizations associated with the given Bible version(s); when omitted,
    /// all organizations are returned.
    ///
    /// # Arguments
    ///
    /// * `bible_ids_array` - Filter organizations to those associated with the given Bible version(s). Use bracket notation: bible_ids[]=111&bible_ids[]=206. When omitted, returns all organizations.
    /// * `page_size` - The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    /// * `fields_array` - A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    /// * `page_token` - The page token to retrieve results from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1organizations_collection_get(
        &self,
        request: &V1OrganizationsCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<V1OrganizationsCollectionGetResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/organizations",
                None,
                QueryBuilder::new()
                    .int_array("bible_ids[]", request.bible_ids_array.clone())
                    .serialize("page_size", request.page_size.clone())
                    .string_array("fields[]", request.fields_array.clone())
                    .string("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a single organization resource by its id.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The Organization unique ID provided in the Platform.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1organizations_resource_get(
        &self,
        organization_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<V1OrganizationsResourceGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/organizations/{}",
                    crate::encode_path_segment(organization_id)?
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get bibles associated with a specific organization by its id.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The Organization unique ID provided in the Platform.
    /// * `page_size` - The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    /// * `fields_array` - A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    /// * `page_token` - The page token to retrieve results from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn v1organizations_bibles_collection_get(
        &self,
        organization_id: &str,
        request: &V1OrganizationsBiblesCollectionGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<V1OrganizationsBiblesCollectionGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/organizations/{}/bibles",
                    crate::encode_path_segment(organization_id)?
                ),
                None,
                QueryBuilder::new()
                    .serialize("page_size", request.page_size.clone())
                    .string_array("fields[]", request.fields_array.clone())
                    .string("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }
}
