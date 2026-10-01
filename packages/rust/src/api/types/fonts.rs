pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Fonts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FontsDataItem>>,
}

impl Fonts {
    pub fn builder() -> FontsBuilder {
        <FontsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FontsBuilder {
    data: Option<Vec<FontsDataItem>>,
}

impl FontsBuilder {
    pub fn data(mut self, value: Vec<FontsDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Fonts`].
    pub fn build(self) -> Result<Fonts, BuildError> {
        Ok(Fonts { data: self.data })
    }
}
