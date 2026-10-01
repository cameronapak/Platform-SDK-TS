pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontSource {
    /// The file format for this source asset.
    pub format: FontSourceFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl FontSource {
    pub fn builder() -> FontSourceBuilder {
        <FontSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontSourceBuilder {
    format: Option<FontSourceFormat>,
    url: Option<String>,
}

impl FontSourceBuilder {
    pub fn format(mut self, value: FontSourceFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FontSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](FontSourceBuilder::format)
    /// - [`url`](FontSourceBuilder::url)
    pub fn build(self) -> Result<FontSource, BuildError> {
        Ok(FontSource {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
