pub use crate::prelude::*;

/// Query parameters for v1HighlightsCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsCollectionGetQueryRequest {
    /// The Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
    /// The passage identifier (verse or chapter USFM format)
    #[serde(default)]
    pub passage_id: String,
}

impl V1HighlightsCollectionGetQueryRequest {
    pub fn builder() -> V1HighlightsCollectionGetQueryRequestBuilder {
        <V1HighlightsCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsCollectionGetQueryRequestBuilder {
    bible_id: Option<i64>,
    passage_id: Option<String>,
}

impl V1HighlightsCollectionGetQueryRequestBuilder {
    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1HighlightsCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bible_id`](V1HighlightsCollectionGetQueryRequestBuilder::bible_id)
    /// - [`passage_id`](V1HighlightsCollectionGetQueryRequestBuilder::passage_id)
    pub fn build(self) -> Result<V1HighlightsCollectionGetQueryRequest, BuildError> {
        Ok(V1HighlightsCollectionGetQueryRequest {
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
        })
    }
}
