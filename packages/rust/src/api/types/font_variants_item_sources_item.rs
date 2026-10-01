pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontVariantsItemSourcesItem {
    /// The file format for this source asset.
    pub format: FontVariantsItemSourcesItemFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl FontVariantsItemSourcesItem {
    pub fn builder() -> FontVariantsItemSourcesItemBuilder {
        <FontVariantsItemSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontVariantsItemSourcesItemBuilder {
    format: Option<FontVariantsItemSourcesItemFormat>,
    url: Option<String>,
}

impl FontVariantsItemSourcesItemBuilder {
    pub fn format(mut self, value: FontVariantsItemSourcesItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FontVariantsItemSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](FontVariantsItemSourcesItemBuilder::format)
    /// - [`url`](FontVariantsItemSourcesItemBuilder::url)
    pub fn build(self) -> Result<FontVariantsItemSourcesItem, BuildError> {
        Ok(FontVariantsItemSourcesItem {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
