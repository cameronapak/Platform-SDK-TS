pub use crate::prelude::*;

/// A font family resource with available variants and CDN-backed source files.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1FontsCollectionGetResponseDataItem {
    /// Stable integer identifier for the font family.
    #[serde(default)]
    pub id: i64,
    /// Stable URL-safe identifier for the font family.
    #[serde(default)]
    pub slug: String,
    /// Canonical font-family name clients should use.
    #[serde(default)]
    pub family: String,
    /// Available faces within this font family.
    #[serde(default)]
    pub variants: Vec<V1FontsCollectionGetResponseDataItemVariantsItem>,
}

impl V1FontsCollectionGetResponseDataItem {
    pub fn builder() -> V1FontsCollectionGetResponseDataItemBuilder {
        <V1FontsCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsCollectionGetResponseDataItemBuilder {
    id: Option<i64>,
    slug: Option<String>,
    family: Option<String>,
    variants: Option<Vec<V1FontsCollectionGetResponseDataItemVariantsItem>>,
}

impl V1FontsCollectionGetResponseDataItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn family(mut self, value: impl Into<String>) -> Self {
        self.family = Some(value.into());
        self
    }

    pub fn variants(
        mut self,
        value: Vec<V1FontsCollectionGetResponseDataItemVariantsItem>,
    ) -> Self {
        self.variants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1FontsCollectionGetResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](V1FontsCollectionGetResponseDataItemBuilder::id)
    /// - [`slug`](V1FontsCollectionGetResponseDataItemBuilder::slug)
    /// - [`family`](V1FontsCollectionGetResponseDataItemBuilder::family)
    /// - [`variants`](V1FontsCollectionGetResponseDataItemBuilder::variants)
    pub fn build(self) -> Result<V1FontsCollectionGetResponseDataItem, BuildError> {
        Ok(V1FontsCollectionGetResponseDataItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            family: self
                .family
                .ok_or_else(|| BuildError::missing_field("family"))?,
            variants: self
                .variants
                .ok_or_else(|| BuildError::missing_field("variants"))?,
        })
    }
}
