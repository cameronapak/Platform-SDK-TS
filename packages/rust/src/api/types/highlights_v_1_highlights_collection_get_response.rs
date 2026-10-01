pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1HighlightsCollectionGetResponseDataItem>>,
}

impl V1HighlightsCollectionGetResponse {
    pub fn builder() -> V1HighlightsCollectionGetResponseBuilder {
        <V1HighlightsCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsCollectionGetResponseBuilder {
    data: Option<Vec<V1HighlightsCollectionGetResponseDataItem>>,
}

impl V1HighlightsCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1HighlightsCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1HighlightsCollectionGetResponse`].
    pub fn build(self) -> Result<V1HighlightsCollectionGetResponse, BuildError> {
        Ok(V1HighlightsCollectionGetResponse { data: self.data })
    }
}
