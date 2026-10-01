use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DataExchangeClient {
    pub http_client: HttpClient,
}

impl DataExchangeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the browser-rendered data exchange approval page for a short-lived token. The app
    /// should first create a token with `POST /data-exchange/token`, then open this URL in the
    /// user's browser with the token and app context. The page lets the user review the requested
    /// permissions before approving or cancelling the exchange.
    ///
    /// # Arguments
    ///
    /// * `token` - Short-lived data exchange token created by `POST /data-exchange/token`.
    /// * `x_yvp_app_key` - Public app key used to resolve the app for direct browser flows.
    /// * `x_yvp_app_id` - App identifier used when a public app key is not supplied.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
    pub async fn approval_get(
        &self,
        request: &ApprovalGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Option<String>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data-exchange",
                None,
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .string("x-yvp-app-key", request.x_yvp_app_key.clone())
                    .string("x-yvp-app-id", request.x_yvp_app_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Completes the browser approval flow and redirects the browser to the app's configured
    /// callback URL. When approval succeeds, the callback receives
    /// `data_exchange_status=granted` and `granted_permissions`. When the user cancels from the
    /// approval page, the callback receives `data_exchange_status=cancelled`,
    /// `denied_permissions`, and `error=access_denied`. Recoverable errors after a safe callback
    /// is known redirect with `data_exchange_status=error`, `error`, and `error_description`.
    ///
    /// # Arguments
    ///
    /// * `token` - Short-lived data exchange token created by `POST /data-exchange/token`.
    /// * `x_yvp_app_key` - Public app key used to resolve the app when a token is not supplied.
    /// * `x_yvp_app_id` - App identifier used when a public app key is not supplied.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn approval_post(
        &self,
        request: &ApprovalPostQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "data-exchange",
                None,
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .string("x-yvp-app-key", request.x_yvp_app_key.clone())
                    .string("x-yvp-app-id", request.x_yvp_app_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a short-lived token that can be passed to the `/data-exchange` browser flow as a
    /// query parameter. Send `requested_permissions` in the request body to declare which
    /// permissions the user should review during the browser flow. Tokens expire after five minutes
    /// and are consumed when the form is submitted.
    ///
    /// # Arguments
    ///
    /// * `x_yvp_app_key` - Public app key used to resolve the app for direct browser flows.
    /// * `x_yvp_app_id` - App identifier used when a public app key is not supplied.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn token_post(
        &self,
        request: &DataExchangeTokenPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<DataExchangeTokenPostResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "data-exchange/token",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("x-yvp-app-key", request.x_yvp_app_key.clone())
                    .string("x-yvp-app-id", request.x_yvp_app_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn approval_get_with_raw_response(
        &self,
        request: &ApprovalGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::RawResponse<Option<String>>, ApiError> {
        self.http_client
            .execute_request_raw(
                Method::GET,
                "data-exchange",
                None,
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .string("x-yvp-app-key", request.x_yvp_app_key.clone())
                    .string("x-yvp-app-id", request.x_yvp_app_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn approval_post_with_raw_response(
        &self,
        request: &ApprovalPostQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<crate::RawResponse<()>, ApiError> {
        self.http_client
            .execute_request_raw(
                Method::POST,
                "data-exchange",
                None,
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .string("x-yvp-app-key", request.x_yvp_app_key.clone())
                    .string("x-yvp-app-id", request.x_yvp_app_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
