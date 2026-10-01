pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Verses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<VersesDataItem>>,
}

impl Verses {
    pub fn builder() -> VersesBuilder {
        <VersesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VersesBuilder {
    data: Option<Vec<VersesDataItem>>,
}

impl VersesBuilder {
    pub fn data(mut self, value: Vec<VersesDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Verses`].
    pub fn build(self) -> Result<Verses, BuildError> {
        Ok(Verses { data: self.data })
    }
}
