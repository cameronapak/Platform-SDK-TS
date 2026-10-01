pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LicensesCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1LicensesCollectionGetResponseDataItem>>,
}

impl V1LicensesCollectionGetResponse {
    pub fn builder() -> V1LicensesCollectionGetResponseBuilder {
        <V1LicensesCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LicensesCollectionGetResponseBuilder {
    data: Option<Vec<V1LicensesCollectionGetResponseDataItem>>,
}

impl V1LicensesCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1LicensesCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1LicensesCollectionGetResponse`].
    pub fn build(self) -> Result<V1LicensesCollectionGetResponse, BuildError> {
        Ok(V1LicensesCollectionGetResponse { data: self.data })
    }
}
