pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchQueriesCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1SearchQueriesCollectionGetResponseDataItem>>,
}

impl V1SearchQueriesCollectionGetResponse {
    pub fn builder() -> V1SearchQueriesCollectionGetResponseBuilder {
        <V1SearchQueriesCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchQueriesCollectionGetResponseBuilder {
    data: Option<Vec<V1SearchQueriesCollectionGetResponseDataItem>>,
}

impl V1SearchQueriesCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1SearchQueriesCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1SearchQueriesCollectionGetResponse`].
    pub fn build(self) -> Result<V1SearchQueriesCollectionGetResponse, BuildError> {
        Ok(V1SearchQueriesCollectionGetResponse { data: self.data })
    }
}
