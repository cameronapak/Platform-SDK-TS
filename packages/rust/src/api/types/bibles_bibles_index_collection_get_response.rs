pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesIndexCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books: Option<Vec<BiblesIndexCollectionGetResponseBooksItem>>,
}

impl BiblesIndexCollectionGetResponse {
    pub fn builder() -> BiblesIndexCollectionGetResponseBuilder {
        <BiblesIndexCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesIndexCollectionGetResponseBuilder {
    text_direction: Option<String>,
    books: Option<Vec<BiblesIndexCollectionGetResponseBooksItem>>,
}

impl BiblesIndexCollectionGetResponseBuilder {
    pub fn text_direction(mut self, value: impl Into<String>) -> Self {
        self.text_direction = Some(value.into());
        self
    }

    pub fn books(mut self, value: Vec<BiblesIndexCollectionGetResponseBooksItem>) -> Self {
        self.books = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BiblesIndexCollectionGetResponse`].
    pub fn build(self) -> Result<BiblesIndexCollectionGetResponse, BuildError> {
        Ok(BiblesIndexCollectionGetResponse {
            text_direction: self.text_direction,
            books: self.books,
        })
    }
}
