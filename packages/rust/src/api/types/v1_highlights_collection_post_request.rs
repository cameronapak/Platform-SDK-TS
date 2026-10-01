pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1HighlightsCollectionPostRequest {
    /// Request UUID for idempotent create retries.
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub highlight: V1HighlightsCollectionPostRequestHighlight,
}

impl V1HighlightsCollectionPostRequest {
    pub fn builder() -> V1HighlightsCollectionPostRequestBuilder {
        <V1HighlightsCollectionPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1HighlightsCollectionPostRequestBuilder {
    request_id: Option<String>,
    highlight: Option<V1HighlightsCollectionPostRequestHighlight>,
}

impl V1HighlightsCollectionPostRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn highlight(mut self, value: V1HighlightsCollectionPostRequestHighlight) -> Self {
        self.highlight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1HighlightsCollectionPostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](V1HighlightsCollectionPostRequestBuilder::request_id)
    /// - [`highlight`](V1HighlightsCollectionPostRequestBuilder::highlight)
    pub fn build(self) -> Result<V1HighlightsCollectionPostRequest, BuildError> {
        Ok(V1HighlightsCollectionPostRequest {
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            highlight: self
                .highlight
                .ok_or_else(|| BuildError::missing_field("highlight"))?,
        })
    }
}
