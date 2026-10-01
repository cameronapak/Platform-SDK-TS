pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1VerseOfTheDaysCanonicalCollectionGetResponse {
    #[serde(default)]
    pub data: Vec<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem>,
}

impl V1VerseOfTheDaysCanonicalCollectionGetResponse {
    pub fn builder() -> V1VerseOfTheDaysCanonicalCollectionGetResponseBuilder {
        <V1VerseOfTheDaysCanonicalCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1VerseOfTheDaysCanonicalCollectionGetResponseBuilder {
    data: Option<Vec<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem>>,
}

impl V1VerseOfTheDaysCanonicalCollectionGetResponseBuilder {
    pub fn data(
        mut self,
        value: Vec<V1VerseOfTheDaysCanonicalCollectionGetResponseDataItem>,
    ) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1VerseOfTheDaysCanonicalCollectionGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](V1VerseOfTheDaysCanonicalCollectionGetResponseBuilder::data)
    pub fn build(self) -> Result<V1VerseOfTheDaysCanonicalCollectionGetResponse, BuildError> {
        Ok(V1VerseOfTheDaysCanonicalCollectionGetResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
