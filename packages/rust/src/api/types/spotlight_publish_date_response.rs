pub use crate::prelude::*;

/// Manifest entry state returned after a publish-date assignment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpotlightPublishDateResponse {
    /// Slug of the updated manifest entry.
    #[serde(default)]
    pub slug: String,
    /// The assigned go-live month (YYYY-MM), or null if cleared.
    #[serde(rename = "publishDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<String>,
}

impl SpotlightPublishDateResponse {
    pub fn builder() -> SpotlightPublishDateResponseBuilder {
        <SpotlightPublishDateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpotlightPublishDateResponseBuilder {
    slug: Option<String>,
    publish_date: Option<String>,
}

impl SpotlightPublishDateResponseBuilder {
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn publish_date(mut self, value: impl Into<String>) -> Self {
        self.publish_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpotlightPublishDateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`slug`](SpotlightPublishDateResponseBuilder::slug)
    pub fn build(self) -> Result<SpotlightPublishDateResponse, BuildError> {
        Ok(SpotlightPublishDateResponse {
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            publish_date: self.publish_date,
        })
    }
}
