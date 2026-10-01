pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsCollectionGetResponseDataItem {
    /// Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
    /// The passage identifier (verse USFM format)
    #[serde(default)]
    pub passage_id: String,
    /// The highlight color in hex format
    #[serde(default)]
    pub color: String,
}

impl V1HighlightsCollectionGetResponseDataItem {
    pub fn builder() -> V1HighlightsCollectionGetResponseDataItemBuilder {
        <V1HighlightsCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsCollectionGetResponseDataItemBuilder {
    bible_id: Option<i64>,
    passage_id: Option<String>,
    color: Option<String>,
}

impl V1HighlightsCollectionGetResponseDataItemBuilder {
    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1HighlightsCollectionGetResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bible_id`](V1HighlightsCollectionGetResponseDataItemBuilder::bible_id)
    /// - [`passage_id`](V1HighlightsCollectionGetResponseDataItemBuilder::passage_id)
    /// - [`color`](V1HighlightsCollectionGetResponseDataItemBuilder::color)
    pub fn build(self) -> Result<V1HighlightsCollectionGetResponseDataItem, BuildError> {
        Ok(V1HighlightsCollectionGetResponseDataItem {
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
            color: self
                .color
                .ok_or_else(|| BuildError::missing_field("color"))?,
        })
    }
}
