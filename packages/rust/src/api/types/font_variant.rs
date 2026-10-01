pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontVariant {
    /// Numeric font weight for this variant.
    #[serde(default)]
    pub weight: i64,
    /// Font style for this variant.
    pub style: FontVariantStyle,
    /// CDN assets available for this specific weight and style.
    #[serde(default)]
    pub sources: Vec<FontVariantSourcesItem>,
}

impl FontVariant {
    pub fn builder() -> FontVariantBuilder {
        <FontVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontVariantBuilder {
    weight: Option<i64>,
    style: Option<FontVariantStyle>,
    sources: Option<Vec<FontVariantSourcesItem>>,
}

impl FontVariantBuilder {
    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn style(mut self, value: FontVariantStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<FontVariantSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FontVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weight`](FontVariantBuilder::weight)
    /// - [`style`](FontVariantBuilder::style)
    /// - [`sources`](FontVariantBuilder::sources)
    pub fn build(self) -> Result<FontVariant, BuildError> {
        Ok(FontVariant {
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
            style: self
                .style
                .ok_or_else(|| BuildError::missing_field("style"))?,
            sources: self
                .sources
                .ok_or_else(|| BuildError::missing_field("sources"))?,
        })
    }
}
