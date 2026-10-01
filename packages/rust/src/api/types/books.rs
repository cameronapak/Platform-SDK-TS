pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Books {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BooksDataItem>>,
}

impl Books {
    pub fn builder() -> BooksBuilder {
        <BooksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksBuilder {
    data: Option<Vec<BooksDataItem>>,
}

impl BooksBuilder {
    pub fn data(mut self, value: Vec<BooksDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Books`].
    pub fn build(self) -> Result<Books, BuildError> {
        Ok(Books { data: self.data })
    }
}
