pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontVariantsItem {
    /// Numeric font weight for this variant.
    #[serde(default)]
    pub weight: i64,
    /// Font style for this variant.
    pub style: FontVariantsItemStyle,
    /// CDN assets available for this specific weight and style.
    #[serde(default)]
    pub sources: Vec<FontVariantsItemSourcesItem>,
}

impl FontVariantsItem {
    pub fn builder() -> FontVariantsItemBuilder {
        <FontVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontVariantsItemBuilder {
    weight: Option<i64>,
    style: Option<FontVariantsItemStyle>,
    sources: Option<Vec<FontVariantsItemSourcesItem>>,
}

impl FontVariantsItemBuilder {
    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn style(mut self, value: FontVariantsItemStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<FontVariantsItemSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FontVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weight`](FontVariantsItemBuilder::weight)
    /// - [`style`](FontVariantsItemBuilder::style)
    /// - [`sources`](FontVariantsItemBuilder::sources)
    pub fn build(self) -> Result<FontVariantsItem, BuildError> {
        Ok(FontVariantsItem {
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
