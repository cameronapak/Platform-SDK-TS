pub use crate::prelude::*;

/// Query parameters for v1LicensesCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LicensesCollectionGetQueryRequest {
    /// The Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
    /// The Developer's unique ID in the Platform.
    #[serde(default)]
    pub developer_id: String,
    /// This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_available: Option<bool>,
}

impl V1LicensesCollectionGetQueryRequest {
    pub fn builder() -> V1LicensesCollectionGetQueryRequestBuilder {
        <V1LicensesCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LicensesCollectionGetQueryRequestBuilder {
    bible_id: Option<i64>,
    developer_id: Option<String>,
    all_available: Option<bool>,
}

impl V1LicensesCollectionGetQueryRequestBuilder {
    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    pub fn developer_id(mut self, value: impl Into<String>) -> Self {
        self.developer_id = Some(value.into());
        self
    }

    pub fn all_available(mut self, value: bool) -> Self {
        self.all_available = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1LicensesCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bible_id`](V1LicensesCollectionGetQueryRequestBuilder::bible_id)
    /// - [`developer_id`](V1LicensesCollectionGetQueryRequestBuilder::developer_id)
    pub fn build(self) -> Result<V1LicensesCollectionGetQueryRequest, BuildError> {
        Ok(V1LicensesCollectionGetQueryRequest {
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
            developer_id: self
                .developer_id
                .ok_or_else(|| BuildError::missing_field("developer_id"))?,
            all_available: self.all_available,
        })
    }
}
