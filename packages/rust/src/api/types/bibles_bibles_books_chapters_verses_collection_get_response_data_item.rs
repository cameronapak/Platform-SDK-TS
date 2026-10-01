pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BiblesBooksChaptersVersesCollectionGetResponseDataItem {
    /// Verse identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Passage identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passage_id: Option<String>,
    /// Verse title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl BiblesBooksChaptersVersesCollectionGetResponseDataItem {
    pub fn builder() -> BiblesBooksChaptersVersesCollectionGetResponseDataItemBuilder {
        <BiblesBooksChaptersVersesCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesBooksChaptersVersesCollectionGetResponseDataItemBuilder {
    id: Option<String>,
    passage_id: Option<String>,
    title: Option<String>,
}

impl BiblesBooksChaptersVersesCollectionGetResponseDataItemBuilder {
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

    /// Consumes the builder and constructs a [`BiblesBooksChaptersVersesCollectionGetResponseDataItem`].
    pub fn build(
        self,
    ) -> Result<BiblesBooksChaptersVersesCollectionGetResponseDataItem, BuildError> {
        Ok(BiblesBooksChaptersVersesCollectionGetResponseDataItem {
            id: self.id,
            passage_id: self.passage_id,
            title: self.title,
        })
    }
}
