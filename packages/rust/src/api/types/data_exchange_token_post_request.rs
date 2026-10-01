pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataExchangeTokenPostRequest {
    /// Data exchange permissions the user should review in the browser flow.
    #[serde(default)]
    pub requested_permissions: Vec<String>,
    /// Public app key used to resolve the app for direct browser flows.
    #[serde(rename = "x-yvp-app-key")]
    #[serde(skip)]
    pub x_yvp_app_key: Option<String>,
    /// App identifier used when a public app key is not supplied.
    #[serde(rename = "x-yvp-app-id")]
    #[serde(skip)]
    pub x_yvp_app_id: Option<String>,
}

impl DataExchangeTokenPostRequest {
    pub fn builder() -> DataExchangeTokenPostRequestBuilder {
        <DataExchangeTokenPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataExchangeTokenPostRequestBuilder {
    requested_permissions: Option<Vec<String>>,
    x_yvp_app_key: Option<String>,
    x_yvp_app_id: Option<String>,
}

impl DataExchangeTokenPostRequestBuilder {
    pub fn requested_permissions(mut self, value: Vec<String>) -> Self {
        self.requested_permissions = Some(value);
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

    /// Consumes the builder and constructs a [`DataExchangeTokenPostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested_permissions`](DataExchangeTokenPostRequestBuilder::requested_permissions)
    pub fn build(self) -> Result<DataExchangeTokenPostRequest, BuildError> {
        Ok(DataExchangeTokenPostRequest {
            requested_permissions: self
                .requested_permissions
                .ok_or_else(|| BuildError::missing_field("requested_permissions"))?,
            x_yvp_app_key: self.x_yvp_app_key,
            x_yvp_app_id: self.x_yvp_app_id,
        })
    }
}
