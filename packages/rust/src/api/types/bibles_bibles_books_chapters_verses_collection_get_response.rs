pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesBooksChaptersVersesCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BiblesBooksChaptersVersesCollectionGetResponseDataItem>>,
}

impl BiblesBooksChaptersVersesCollectionGetResponse {
    pub fn builder() -> BiblesBooksChaptersVersesCollectionGetResponseBuilder {
        <BiblesBooksChaptersVersesCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesBooksChaptersVersesCollectionGetResponseBuilder {
    data: Option<Vec<BiblesBooksChaptersVersesCollectionGetResponseDataItem>>,
}

impl BiblesBooksChaptersVersesCollectionGetResponseBuilder {
    pub fn data(
        mut self,
        value: Vec<BiblesBooksChaptersVersesCollectionGetResponseDataItem>,
    ) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BiblesBooksChaptersVersesCollectionGetResponse`].
    pub fn build(self) -> Result<BiblesBooksChaptersVersesCollectionGetResponse, BuildError> {
        Ok(BiblesBooksChaptersVersesCollectionGetResponse { data: self.data })
    }
}
