pub use crate::prelude::*;

/// A font family resource with available variants and CDN-backed source files.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Font {
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
    pub variants: Vec<FontVariantsItem>,
}

impl Font {
    pub fn builder() -> FontBuilder {
        <FontBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontBuilder {
    id: Option<i64>,
    slug: Option<String>,
    family: Option<String>,
    variants: Option<Vec<FontVariantsItem>>,
}

impl FontBuilder {
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

    pub fn variants(mut self, value: Vec<FontVariantsItem>) -> Self {
        self.variants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Font`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FontBuilder::id)
    /// - [`slug`](FontBuilder::slug)
    /// - [`family`](FontBuilder::family)
    /// - [`variants`](FontBuilder::variants)
    pub fn build(self) -> Result<Font, BuildError> {
        Ok(Font {
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
