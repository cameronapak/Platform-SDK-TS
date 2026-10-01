pub use crate::prelude::*;

/// Query parameters for approvalGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalGetQueryRequest {
    /// Short-lived data exchange token created by `POST /data-exchange/token`.
    #[serde(default)]
    pub token: String,
    /// Public app key used to resolve the app for direct browser flows.
    #[serde(rename = "x-yvp-app-key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_yvp_app_key: Option<String>,
    /// App identifier used when a public app key is not supplied.
    #[serde(rename = "x-yvp-app-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_yvp_app_id: Option<String>,
}

impl ApprovalGetQueryRequest {
    pub fn builder() -> ApprovalGetQueryRequestBuilder {
        <ApprovalGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalGetQueryRequestBuilder {
    token: Option<String>,
    x_yvp_app_key: Option<String>,
    x_yvp_app_id: Option<String>,
}

impl ApprovalGetQueryRequestBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn x_yvp_app_key(mut self, value: impl Into<String>) -> Self {
        self.x_yvp_app_key = Some(value.into());
        self
    }

    pub fn x_yvp_app_id(mut self, value: impl Into<String>) -> Self {
        self.x_yvp_app_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](ApprovalGetQueryRequestBuilder::token)
    pub fn build(self) -> Result<ApprovalGetQueryRequest, BuildError> {
        Ok(ApprovalGetQueryRequest {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            x_yvp_app_key: self.x_yvp_app_key,
            x_yvp_app_id: self.x_yvp_app_id,
        })
    }
}
