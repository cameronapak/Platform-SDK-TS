pub use crate::prelude::*;

/// Assigns (or clears) a spotlight manifest entry's go-live month.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpotlightPublishDateRequest {
    /// Go-live month as YYYY-MM; null clears the assignment.
    #[serde(rename = "publishDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<String>,
}

impl SpotlightPublishDateRequest {
    pub fn builder() -> SpotlightPublishDateRequestBuilder {
        <SpotlightPublishDateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpotlightPublishDateRequestBuilder {
    publish_date: Option<String>,
}

impl SpotlightPublishDateRequestBuilder {
    pub fn publish_date(mut self, value: impl Into<String>) -> Self {
        self.publish_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpotlightPublishDateRequest`].
    pub fn build(self) -> Result<SpotlightPublishDateRequest, BuildError> {
        Ok(SpotlightPublishDateRequest {
            publish_date: self.publish_date,
        })
    }
}
