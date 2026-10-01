pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FontsDataItemVariantsItem {
    /// Numeric font weight for this variant.
    #[serde(default)]
    pub weight: i64,
    /// Font style for this variant.
    pub style: FontsDataItemVariantsItemStyle,
    /// CDN assets available for this specific weight and style.
    #[serde(default)]
    pub sources: Vec<FontsDataItemVariantsItemSourcesItem>,
}

impl FontsDataItemVariantsItem {
    pub fn builder() -> FontsDataItemVariantsItemBuilder {
        <FontsDataItemVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontsDataItemVariantsItemBuilder {
    weight: Option<i64>,
    style: Option<FontsDataItemVariantsItemStyle>,
    sources: Option<Vec<FontsDataItemVariantsItemSourcesItem>>,
}

impl FontsDataItemVariantsItemBuilder {
    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn style(mut self, value: FontsDataItemVariantsItemStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<FontsDataItemVariantsItemSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FontsDataItemVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weight`](FontsDataItemVariantsItemBuilder::weight)
    /// - [`style`](FontsDataItemVariantsItemBuilder::style)
    /// - [`sources`](FontsDataItemVariantsItemBuilder::sources)
    pub fn build(self) -> Result<FontsDataItemVariantsItem, BuildError> {
        Ok(FontsDataItemVariantsItem {
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
