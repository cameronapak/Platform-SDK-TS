pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Highlights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HighlightsDataItem>>,
}

impl Highlights {
    pub fn builder() -> HighlightsBuilder {
        <HighlightsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HighlightsBuilder {
    data: Option<Vec<HighlightsDataItem>>,
}

impl HighlightsBuilder {
    pub fn data(mut self, value: Vec<HighlightsDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Highlights`].
    pub fn build(self) -> Result<Highlights, BuildError> {
        Ok(Highlights { data: self.data })
    }
}
