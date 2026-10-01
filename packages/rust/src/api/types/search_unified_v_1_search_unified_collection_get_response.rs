pub use crate::prelude::*;

/// A single unified, unpaginated set of search results grouped by kind: verse references and related topics. Returns one combined page of top results per kind; there is no page_size or page_token. Use the kind-specific endpoints (search-verses, search-topics) to page through the full result set for a kind.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchUnifiedCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verses: Option<Vec<V1SearchUnifiedCollectionGetResponseVersesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<V1SearchUnifiedCollectionGetResponseTopicsItem>>,
    /// The intent the Core Search service resolved for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_intent: Option<String>,
    /// Alternative spellings the search service suggests for the query.
    #[serde(default)]
    pub did_you_mean: Vec<String>,
    /// A corrected query the results were actually returned for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_instead_for: Option<String>,
}

impl V1SearchUnifiedCollectionGetResponse {
    pub fn builder() -> V1SearchUnifiedCollectionGetResponseBuilder {
        <V1SearchUnifiedCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchUnifiedCollectionGetResponseBuilder {
    verses: Option<Vec<V1SearchUnifiedCollectionGetResponseVersesItem>>,
    topics: Option<Vec<V1SearchUnifiedCollectionGetResponseTopicsItem>>,
    user_intent: Option<String>,
    did_you_mean: Option<Vec<String>>,
    search_instead_for: Option<String>,
}

impl V1SearchUnifiedCollectionGetResponseBuilder {
    pub fn verses(mut self, value: Vec<V1SearchUnifiedCollectionGetResponseVersesItem>) -> Self {
        self.verses = Some(value);
        self
    }

    pub fn topics(mut self, value: Vec<V1SearchUnifiedCollectionGetResponseTopicsItem>) -> Self {
        self.topics = Some(value);
        self
    }

    pub fn user_intent(mut self, value: impl Into<String>) -> Self {
        self.user_intent = Some(value.into());
        self
    }

    pub fn did_you_mean(mut self, value: Vec<String>) -> Self {
        self.did_you_mean = Some(value);
        self
    }

    pub fn search_instead_for(mut self, value: impl Into<String>) -> Self {
        self.search_instead_for = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1SearchUnifiedCollectionGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`did_you_mean`](V1SearchUnifiedCollectionGetResponseBuilder::did_you_mean)
    pub fn build(self) -> Result<V1SearchUnifiedCollectionGetResponse, BuildError> {
        Ok(V1SearchUnifiedCollectionGetResponse {
            verses: self.verses,
            topics: self.topics,
            user_intent: self.user_intent,
            did_you_mean: self
                .did_you_mean
                .ok_or_else(|| BuildError::missing_field("did_you_mean"))?,
            search_instead_for: self.search_instead_for,
        })
    }
}
