pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1VerseOfTheDaysCanonicalResourceGetResponse {
    #[serde(default)]
    pub day: i64,
    /// The passage identifier
    #[serde(default)]
    pub passage_id: String,
}

impl V1VerseOfTheDaysCanonicalResourceGetResponse {
    pub fn builder() -> V1VerseOfTheDaysCanonicalResourceGetResponseBuilder {
        <V1VerseOfTheDaysCanonicalResourceGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1VerseOfTheDaysCanonicalResourceGetResponseBuilder {
    day: Option<i64>,
    passage_id: Option<String>,
}

impl V1VerseOfTheDaysCanonicalResourceGetResponseBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1VerseOfTheDaysCanonicalResourceGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](V1VerseOfTheDaysCanonicalResourceGetResponseBuilder::day)
    /// - [`passage_id`](V1VerseOfTheDaysCanonicalResourceGetResponseBuilder::passage_id)
    pub fn build(self) -> Result<V1VerseOfTheDaysCanonicalResourceGetResponse, BuildError> {
        Ok(V1VerseOfTheDaysCanonicalResourceGetResponse {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
        })
    }
}
