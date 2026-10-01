pub use crate::prelude::*;

/// Query parameters for approvalPost
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalPostQueryRequest {
    /// Short-lived data exchange token created by `POST /data-exchange/token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Public app key used to resolve the app when a token is not supplied.
    #[serde(rename = "x-yvp-app-key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_yvp_app_key: Option<String>,
    /// App identifier used when a public app key is not supplied.
    #[serde(rename = "x-yvp-app-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_yvp_app_id: Option<String>,
}

impl ApprovalPostQueryRequest {
    pub fn builder() -> ApprovalPostQueryRequestBuilder {
        <ApprovalPostQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalPostQueryRequestBuilder {
    token: Option<String>,
    x_yvp_app_key: Option<String>,
    x_yvp_app_id: Option<String>,
}

impl ApprovalPostQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ApprovalPostQueryRequest`].
    pub fn build(self) -> Result<ApprovalPostQueryRequest, BuildError> {
        Ok(ApprovalPostQueryRequest {
            token: self.token,
            x_yvp_app_key: self.x_yvp_app_key,
            x_yvp_app_id: self.x_yvp_app_id,
        })
    }
}
