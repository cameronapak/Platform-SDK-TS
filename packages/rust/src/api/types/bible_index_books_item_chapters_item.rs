pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BibleIndexBooksItemChaptersItem {
    /// Chapter identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Passage identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passage_id: Option<String>,
    /// Chapter title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Verses in the chapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verses: Option<Vec<BibleIndexBooksItemChaptersItemVersesItem>>,
}

impl BibleIndexBooksItemChaptersItem {
    pub fn builder() -> BibleIndexBooksItemChaptersItemBuilder {
        <BibleIndexBooksItemChaptersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BibleIndexBooksItemChaptersItemBuilder {
    id: Option<String>,
    passage_id: Option<String>,
    title: Option<String>,
    verses: Option<Vec<BibleIndexBooksItemChaptersItemVersesItem>>,
}

impl BibleIndexBooksItemChaptersItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn verses(mut self, value: Vec<BibleIndexBooksItemChaptersItemVersesItem>) -> Self {
        self.verses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BibleIndexBooksItemChaptersItem`].
    pub fn build(self) -> Result<BibleIndexBooksItemChaptersItem, BuildError> {
        Ok(BibleIndexBooksItemChaptersItem {
            id: self.id,
            passage_id: self.passage_id,
            title: self.title,
            verses: self.verses,
        })
    }
}
