pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1AppsPermissionsCollectionGetResponse {
    /// Permissions the user has granted to the calling app.
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl V1AppsPermissionsCollectionGetResponse {
    pub fn builder() -> V1AppsPermissionsCollectionGetResponseBuilder {
        <V1AppsPermissionsCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1AppsPermissionsCollectionGetResponseBuilder {
    permissions: Option<Vec<String>>,
}

impl V1AppsPermissionsCollectionGetResponseBuilder {
    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1AppsPermissionsCollectionGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](V1AppsPermissionsCollectionGetResponseBuilder::permissions)
    pub fn build(self) -> Result<V1AppsPermissionsCollectionGetResponse, BuildError> {
        Ok(V1AppsPermissionsCollectionGetResponse {
            permissions: self
                .permissions
                .ok_or_else(|| BuildError::missing_field("permissions"))?,
        })
    }
}
