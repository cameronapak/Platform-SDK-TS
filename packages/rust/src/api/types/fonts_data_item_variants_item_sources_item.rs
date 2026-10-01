pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontsDataItemVariantsItemSourcesItem {
    /// The file format for this source asset.
    pub format: FontsDataItemVariantsItemSourcesItemFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl FontsDataItemVariantsItemSourcesItem {
    pub fn builder() -> FontsDataItemVariantsItemSourcesItemBuilder {
        <FontsDataItemVariantsItemSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontsDataItemVariantsItemSourcesItemBuilder {
    format: Option<FontsDataItemVariantsItemSourcesItemFormat>,
    url: Option<String>,
}

impl FontsDataItemVariantsItemSourcesItemBuilder {
    pub fn format(mut self, value: FontsDataItemVariantsItemSourcesItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FontsDataItemVariantsItemSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](FontsDataItemVariantsItemSourcesItemBuilder::format)
    /// - [`url`](FontsDataItemVariantsItemSourcesItemBuilder::url)
    pub fn build(self) -> Result<FontsDataItemVariantsItemSourcesItem, BuildError> {
        Ok(FontsDataItemVariantsItemSourcesItem {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
