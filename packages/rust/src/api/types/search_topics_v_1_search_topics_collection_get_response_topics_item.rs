pub use crate::prelude::*;

/// A topic related to the query, for pivoting to other verses in the same topic.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchTopicsCollectionGetResponseTopicsItem {
    /// The topic's identifier in the Core Search catalog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The topic label.
    #[serde(default)]
    pub text: String,
    /// Related subtopic labels, if any.
    #[serde(default)]
    pub subtopics: Vec<String>,
}

impl V1SearchTopicsCollectionGetResponseTopicsItem {
    pub fn builder() -> V1SearchTopicsCollectionGetResponseTopicsItemBuilder {
        <V1SearchTopicsCollectionGetResponseTopicsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchTopicsCollectionGetResponseTopicsItemBuilder {
    id: Option<i64>,
    text: Option<String>,
    subtopics: Option<Vec<String>>,
}

impl V1SearchTopicsCollectionGetResponseTopicsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn subtopics(mut self, value: Vec<String>) -> Self {
        self.subtopics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1SearchTopicsCollectionGetResponseTopicsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](V1SearchTopicsCollectionGetResponseTopicsItemBuilder::text)
    /// - [`subtopics`](V1SearchTopicsCollectionGetResponseTopicsItemBuilder::subtopics)
    pub fn build(self) -> Result<V1SearchTopicsCollectionGetResponseTopicsItem, BuildError> {
        Ok(V1SearchTopicsCollectionGetResponseTopicsItem {
            id: self.id,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            subtopics: self
                .subtopics
                .ok_or_else(|| BuildError::missing_field("subtopics"))?,
        })
    }
}
