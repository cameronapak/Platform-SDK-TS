pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontVariantSourcesItem {
    /// The file format for this source asset.
    pub format: FontVariantSourcesItemFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl FontVariantSourcesItem {
    pub fn builder() -> FontVariantSourcesItemBuilder {
        <FontVariantSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontVariantSourcesItemBuilder {
    format: Option<FontVariantSourcesItemFormat>,
    url: Option<String>,
}

impl FontVariantSourcesItemBuilder {
    pub fn format(mut self, value: FontVariantSourcesItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FontVariantSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](FontVariantSourcesItemBuilder::format)
    /// - [`url`](FontVariantSourcesItemBuilder::url)
    pub fn build(self) -> Result<FontVariantSourcesItem, BuildError> {
        Ok(FontVariantSourcesItem {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
