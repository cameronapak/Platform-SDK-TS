pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesBooksCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BiblesBooksCollectionGetResponseDataItem>>,
}

impl BiblesBooksCollectionGetResponse {
    pub fn builder() -> BiblesBooksCollectionGetResponseBuilder {
        <BiblesBooksCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesBooksCollectionGetResponseBuilder {
    data: Option<Vec<BiblesBooksCollectionGetResponseDataItem>>,
}

impl BiblesBooksCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<BiblesBooksCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BiblesBooksCollectionGetResponse`].
    pub fn build(self) -> Result<BiblesBooksCollectionGetResponse, BuildError> {
        Ok(BiblesBooksCollectionGetResponse { data: self.data })
    }
}
