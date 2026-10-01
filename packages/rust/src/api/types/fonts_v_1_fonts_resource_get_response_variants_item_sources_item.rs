pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1FontsResourceGetResponseVariantsItemSourcesItem {
    /// The file format for this source asset.
    pub format: V1FontsResourceGetResponseVariantsItemSourcesItemFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl V1FontsResourceGetResponseVariantsItemSourcesItem {
    pub fn builder() -> V1FontsResourceGetResponseVariantsItemSourcesItemBuilder {
        <V1FontsResourceGetResponseVariantsItemSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsResourceGetResponseVariantsItemSourcesItemBuilder {
    format: Option<V1FontsResourceGetResponseVariantsItemSourcesItemFormat>,
    url: Option<String>,
}

impl V1FontsResourceGetResponseVariantsItemSourcesItemBuilder {
    pub fn format(
        mut self,
        value: V1FontsResourceGetResponseVariantsItemSourcesItemFormat,
    ) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1FontsResourceGetResponseVariantsItemSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](V1FontsResourceGetResponseVariantsItemSourcesItemBuilder::format)
    /// - [`url`](V1FontsResourceGetResponseVariantsItemSourcesItemBuilder::url)
    pub fn build(self) -> Result<V1FontsResourceGetResponseVariantsItemSourcesItem, BuildError> {
        Ok(V1FontsResourceGetResponseVariantsItemSourcesItem {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
