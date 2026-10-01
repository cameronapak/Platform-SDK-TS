pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerseOfTheDay {
    #[serde(default)]
    pub day: i64,
    /// The passage identifier
    #[serde(default)]
    pub passage_id: String,
}

impl VerseOfTheDay {
    pub fn builder() -> VerseOfTheDayBuilder {
        <VerseOfTheDayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerseOfTheDayBuilder {
    day: Option<i64>,
    passage_id: Option<String>,
}

impl VerseOfTheDayBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerseOfTheDay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VerseOfTheDayBuilder::day)
    /// - [`passage_id`](VerseOfTheDayBuilder::passage_id)
    pub fn build(self) -> Result<VerseOfTheDay, BuildError> {
        Ok(VerseOfTheDay {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
        })
    }
}
