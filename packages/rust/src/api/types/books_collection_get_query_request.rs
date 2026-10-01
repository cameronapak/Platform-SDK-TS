pub use crate::prelude::*;

/// Query parameters for booksCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksCollectionGetQueryRequest {
    /// The Canon to filter results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canon: Option<BiblesBooksCollectionGetRequestCanon>,
}

impl BooksCollectionGetQueryRequest {
    pub fn builder() -> BooksCollectionGetQueryRequestBuilder {
        <BooksCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksCollectionGetQueryRequestBuilder {
    canon: Option<BiblesBooksCollectionGetRequestCanon>,
}

impl BooksCollectionGetQueryRequestBuilder {
    pub fn canon(mut self, value: BiblesBooksCollectionGetRequestCanon) -> Self {
        self.canon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksCollectionGetQueryRequest`].
    pub fn build(self) -> Result<BooksCollectionGetQueryRequest, BuildError> {
        Ok(BooksCollectionGetQueryRequest { canon: self.canon })
    }
}
