pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerseOfTheDaysDataItem {
    #[serde(default)]
    pub day: i64,
    /// The passage identifier
    #[serde(default)]
    pub passage_id: String,
}

impl VerseOfTheDaysDataItem {
    pub fn builder() -> VerseOfTheDaysDataItemBuilder {
        <VerseOfTheDaysDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerseOfTheDaysDataItemBuilder {
    day: Option<i64>,
    passage_id: Option<String>,
}

impl VerseOfTheDaysDataItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerseOfTheDaysDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VerseOfTheDaysDataItemBuilder::day)
    /// - [`passage_id`](VerseOfTheDaysDataItemBuilder::passage_id)
    pub fn build(self) -> Result<VerseOfTheDaysDataItem, BuildError> {
        Ok(VerseOfTheDaysDataItem {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
        })
    }
}
