pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem {
    #[serde(default)]
    pub day: i64,
    /// The passage identifier
    #[serde(default)]
    pub passage_id: String,
}

impl V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem {
    pub fn builder() -> V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder {
        <V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder {
    day: Option<i64>,
    passage_id: Option<String>,
}

impl V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn passage_id(mut self, value: impl Into<String>) -> Self {
        self.passage_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder::day)
    /// - [`passage_id`](V1VerseOfTheDaysCanonicalCollectionGetResponseDataItemBuilder::passage_id)
    pub fn build(
        self,
    ) -> Result<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem, BuildError> {
        Ok(V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            passage_id: self
                .passage_id
                .ok_or_else(|| BuildError::missing_field("passage_id"))?,
        })
    }
}
