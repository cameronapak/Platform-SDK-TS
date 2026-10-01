pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsCollectionPostResponse {
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

impl V1HighlightsCollectionPostResponse {
    pub fn builder() -> V1HighlightsCollectionPostResponseBuilder {
        <V1HighlightsCollectionPostResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsCollectionPostResponseBuilder {
    bible_id: Option<i64>,
    passage_id: Option<String>,
    color: Option<String>,
}

impl V1HighlightsCollectionPostResponseBuilder {
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

    /// Consumes the builder and constructs a [`V1HighlightsCollectionPostResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bible_id`](V1HighlightsCollectionPostResponseBuilder::bible_id)
    /// - [`passage_id`](V1HighlightsCollectionPostResponseBuilder::passage_id)
    /// - [`color`](V1HighlightsCollectionPostResponseBuilder::color)
    pub fn build(self) -> Result<V1HighlightsCollectionPostResponse, BuildError> {
        Ok(V1HighlightsCollectionPostResponse {
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
