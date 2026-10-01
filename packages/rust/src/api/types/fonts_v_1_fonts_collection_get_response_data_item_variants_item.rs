pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1FontsCollectionGetResponseDataItemVariantsItem {
    /// Numeric font weight for this variant.
    #[serde(default)]
    pub weight: i64,
    /// Font style for this variant.
    pub style: V1FontsCollectionGetResponseDataItemVariantsItemStyle,
    /// CDN assets available for this specific weight and style.
    #[serde(default)]
    pub sources: Vec<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem>,
}

impl V1FontsCollectionGetResponseDataItemVariantsItem {
    pub fn builder() -> V1FontsCollectionGetResponseDataItemVariantsItemBuilder {
        <V1FontsCollectionGetResponseDataItemVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsCollectionGetResponseDataItemVariantsItemBuilder {
    weight: Option<i64>,
    style: Option<V1FontsCollectionGetResponseDataItemVariantsItemStyle>,
    sources: Option<Vec<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem>>,
}

impl V1FontsCollectionGetResponseDataItemVariantsItemBuilder {
    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn style(mut self, value: V1FontsCollectionGetResponseDataItemVariantsItemStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn sources(
        mut self,
        value: Vec<V1FontsCollectionGetResponseDataItemVariantsItemSourcesItem>,
    ) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1FontsCollectionGetResponseDataItemVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weight`](V1FontsCollectionGetResponseDataItemVariantsItemBuilder::weight)
    /// - [`style`](V1FontsCollectionGetResponseDataItemVariantsItemBuilder::style)
    /// - [`sources`](V1FontsCollectionGetResponseDataItemVariantsItemBuilder::sources)
    pub fn build(self) -> Result<V1FontsCollectionGetResponseDataItemVariantsItem, BuildError> {
        Ok(V1FontsCollectionGetResponseDataItemVariantsItem {
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
