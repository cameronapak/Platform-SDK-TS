pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Chapters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ChaptersDataItem>>,
}

impl Chapters {
    pub fn builder() -> ChaptersBuilder {
        <ChaptersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChaptersBuilder {
    data: Option<Vec<ChaptersDataItem>>,
}

impl ChaptersBuilder {
    pub fn data(mut self, value: Vec<ChaptersDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Chapters`].
    pub fn build(self) -> Result<Chapters, BuildError> {
        Ok(Chapters { data: self.data })
    }
}
