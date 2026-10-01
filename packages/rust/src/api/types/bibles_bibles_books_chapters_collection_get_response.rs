pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesBooksChaptersCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BiblesBooksChaptersCollectionGetResponseDataItem>>,
}

impl BiblesBooksChaptersCollectionGetResponse {
    pub fn builder() -> BiblesBooksChaptersCollectionGetResponseBuilder {
        <BiblesBooksChaptersCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesBooksChaptersCollectionGetResponseBuilder {
    data: Option<Vec<BiblesBooksChaptersCollectionGetResponseDataItem>>,
}

impl BiblesBooksChaptersCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<BiblesBooksChaptersCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BiblesBooksChaptersCollectionGetResponse`].
    pub fn build(self) -> Result<BiblesBooksChaptersCollectionGetResponse, BuildError> {
        Ok(BiblesBooksChaptersCollectionGetResponse { data: self.data })
    }
}
