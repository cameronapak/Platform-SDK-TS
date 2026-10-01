pub use crate::prelude::*;

/// Query parameters for v1HighlightsResourceDelete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsResourceDeleteQueryRequest {
    /// The Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
}

impl V1HighlightsResourceDeleteQueryRequest {
    pub fn builder() -> V1HighlightsResourceDeleteQueryRequestBuilder {
        <V1HighlightsResourceDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsResourceDeleteQueryRequestBuilder {
    bible_id: Option<i64>,
}

impl V1HighlightsResourceDeleteQueryRequestBuilder {
    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1HighlightsResourceDeleteQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bible_id`](V1HighlightsResourceDeleteQueryRequestBuilder::bible_id)
    pub fn build(self) -> Result<V1HighlightsResourceDeleteQueryRequest, BuildError> {
        Ok(V1HighlightsResourceDeleteQueryRequest {
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
        })
    }
}
