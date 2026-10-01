pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BibleIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books: Option<Vec<BibleIndexBooksItem>>,
}

impl BibleIndex {
    pub fn builder() -> BibleIndexBuilder {
        <BibleIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BibleIndexBuilder {
    text_direction: Option<String>,
    books: Option<Vec<BibleIndexBooksItem>>,
}

impl BibleIndexBuilder {
    pub fn text_direction(mut self, value: impl Into<String>) -> Self {
        self.text_direction = Some(value.into());
        self
    }

    pub fn books(mut self, value: Vec<BibleIndexBooksItem>) -> Self {
        self.books = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BibleIndex`].
    pub fn build(self) -> Result<BibleIndex, BuildError> {
        Ok(BibleIndex {
            text_direction: self.text_direction,
            books: self.books,
        })
    }
}
