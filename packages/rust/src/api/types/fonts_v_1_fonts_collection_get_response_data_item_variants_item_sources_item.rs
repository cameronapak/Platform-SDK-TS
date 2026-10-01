pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem {
    /// The file format for this source asset.
    pub format: V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat,
    /// Fully qualified CDN URL for this font source file.
    #[serde(default)]
    pub url: String,
}

impl V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem {
    pub fn builder() -> V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder {
        <V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder {
    format: Option<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat>,
    url: Option<String>,
}

impl V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder {
    pub fn format(
        mut self,
        value: V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat,
    ) -> Self {
        self.format = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder::format)
    /// - [`url`](V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemBuilder::url)
    pub fn build(
        self,
    ) -> Result<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem, BuildError> {
        Ok(
            V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem {
                format: self
                    .format
                    .ok_or_else(|| BuildError::missing_field("format"))?,
                url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            },
        )
    }
}
