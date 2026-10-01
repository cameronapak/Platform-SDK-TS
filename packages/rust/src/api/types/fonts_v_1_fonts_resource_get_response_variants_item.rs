pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct V1FontsResourceGetResponseVariantsItem {
    /// Numeric font weight for this variant.
    #[serde(default)]
    pub weight: i64,
    /// Font style for this variant.
    pub style: V1FontsResourceGetResponseVariantsItemStyle,
    /// CDN assets available for this specific weight and style.
    #[serde(default)]
    pub sources: Vec<V1FontsResourceGetResponseVariantsItemSourcesItem>,
}

impl V1FontsResourceGetResponseVariantsItem {
    pub fn builder() -> V1FontsResourceGetResponseVariantsItemBuilder {
        <V1FontsResourceGetResponseVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1FontsResourceGetResponseVariantsItemBuilder {
    weight: Option<i64>,
    style: Option<V1FontsResourceGetResponseVariantsItemStyle>,
    sources: Option<Vec<V1FontsResourceGetResponseVariantsItemSourcesItem>>,
}

impl V1FontsResourceGetResponseVariantsItemBuilder {
    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn style(mut self, value: V1FontsResourceGetResponseVariantsItemStyle) -> Self {
        self.style = Some(value);
        self
    }

    pub fn sources(
        mut self,
        value: Vec<V1FontsResourceGetResponseVariantsItemSourcesItem>,
    ) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1FontsResourceGetResponseVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weight`](V1FontsResourceGetResponseVariantsItemBuilder::weight)
    /// - [`style`](V1FontsResourceGetResponseVariantsItemBuilder::style)
    /// - [`sources`](V1FontsResourceGetResponseVariantsItemBuilder::sources)
    pub fn build(self) -> Result<V1FontsResourceGetResponseVariantsItem, BuildError> {
        Ok(V1FontsResourceGetResponseVariantsItem {
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
