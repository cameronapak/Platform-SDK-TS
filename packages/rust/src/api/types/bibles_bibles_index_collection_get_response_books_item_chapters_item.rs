pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesIndexCollectionGetResponseBooksItemChaptersItem {
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
    pub verses: Option<Vec<BiblesIndexCollectionGetResponseBooksItemChaptersItemVersesItem>>,
}

impl BiblesIndexCollectionGetResponseBooksItemChaptersItem {
    pub fn builder() -> BiblesIndexCollectionGetResponseBooksItemChaptersItemBuilder {
        <BiblesIndexCollectionGetResponseBooksItemChaptersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesIndexCollectionGetResponseBooksItemChaptersItemBuilder {
    id: Option<String>,
    passage_id: Option<String>,
    title: Option<String>,
    verses: Option<Vec<BiblesIndexCollectionGetResponseBooksItemChaptersItemVersesItem>>,
}

impl BiblesIndexCollectionGetResponseBooksItemChaptersItemBuilder {
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

    pub fn verses(
        mut self,
        value: Vec<BiblesIndexCollectionGetResponseBooksItemChaptersItemVersesItem>,
    ) -> Self {
        self.verses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BiblesIndexCollectionGetResponseBooksItemChaptersItem`].
    pub fn build(
        self,
    ) -> Result<BiblesIndexCollectionGetResponseBooksItemChaptersItem, BuildError> {
        Ok(BiblesIndexCollectionGetResponseBooksItemChaptersItem {
            id: self.id,
            passage_id: self.passage_id,
            title: self.title,
            verses: self.verses,
        })
    }
}
