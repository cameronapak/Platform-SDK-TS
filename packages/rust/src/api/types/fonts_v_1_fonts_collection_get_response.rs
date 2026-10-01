pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1FontsCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1FontsCollectionGetResponseDataItem>>,
}

impl V1FontsCollectionGetResponse {
    pub fn builder() -> V1FontsCollectionGetResponseBuilder {
        <V1FontsCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsCollectionGetResponseBuilder {
    data: Option<Vec<V1FontsCollectionGetResponseDataItem>>,
}

impl V1FontsCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1FontsCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1FontsCollectionGetResponse`].
    pub fn build(self) -> Result<V1FontsCollectionGetResponse, BuildError> {
        Ok(V1FontsCollectionGetResponse { data: self.data })
    }
}
