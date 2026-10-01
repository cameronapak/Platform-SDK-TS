pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerseOfTheDays {
    #[serde(default)]
    pub data: Vec<VerseOfTheDaysDataItem>,
}

impl VerseOfTheDays {
    pub fn builder() -> VerseOfTheDaysBuilder {
        <VerseOfTheDaysBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerseOfTheDaysBuilder {
    data: Option<Vec<VerseOfTheDaysDataItem>>,
}

impl VerseOfTheDaysBuilder {
    pub fn data(mut self, value: Vec<VerseOfTheDaysDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VerseOfTheDays`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](VerseOfTheDaysBuilder::data)
    pub fn build(self) -> Result<VerseOfTheDays, BuildError> {
        Ok(VerseOfTheDays {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
