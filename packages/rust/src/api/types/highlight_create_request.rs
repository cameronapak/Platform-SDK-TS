pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HighlightCreateRequest {
    /// Request UUID for idempotent create retries.
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub highlight: HighlightCreateRequestHighlight,
}

impl HighlightCreateRequest {
    pub fn builder() -> HighlightCreateRequestBuilder {
        <HighlightCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HighlightCreateRequestBuilder {
    request_id: Option<String>,
    highlight: Option<HighlightCreateRequestHighlight>,
}

impl HighlightCreateRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn highlight(mut self, value: HighlightCreateRequestHighlight) -> Self {
        self.highlight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HighlightCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](HighlightCreateRequestBuilder::request_id)
    /// - [`highlight`](HighlightCreateRequestBuilder::highlight)
    pub fn build(self) -> Result<HighlightCreateRequest, BuildError> {
        Ok(HighlightCreateRequest {
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            highlight: self
                .highlight
                .ok_or_else(|| BuildError::missing_field("highlight"))?,
        })
    }
}
