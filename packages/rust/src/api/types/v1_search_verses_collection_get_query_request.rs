pub use crate::prelude::*;

/// Query parameters for v1SearchVersesCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchVersesCollectionGetQueryRequest {
    /// The search query string used to find matching results.
    #[serde(default)]
    pub query: String,
    /// The Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
    /// The searcher's intent. Defaults to unknown, matching the Core Search service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_intent: Option<V1SearchVersesCollectionGetRequestUserIntent>,
    /// The number of verse results to return in the collection. Must be between 1 and 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// The page token to retrieve results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl V1SearchVersesCollectionGetQueryRequest {
    pub fn builder() -> V1SearchVersesCollectionGetQueryRequestBuilder {
        <V1SearchVersesCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchVersesCollectionGetQueryRequestBuilder {
    query: Option<String>,
    bible_id: Option<i64>,
    user_intent: Option<V1SearchVersesCollectionGetRequestUserIntent>,
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl V1SearchVersesCollectionGetQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    pub fn user_intent(mut self, value: V1SearchVersesCollectionGetRequestUserIntent) -> Self {
        self.user_intent = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1SearchVersesCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](V1SearchVersesCollectionGetQueryRequestBuilder::query)
    /// - [`bible_id`](V1SearchVersesCollectionGetQueryRequestBuilder::bible_id)
    pub fn build(self) -> Result<V1SearchVersesCollectionGetQueryRequest, BuildError> {
        Ok(V1SearchVersesCollectionGetQueryRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
            user_intent: self.user_intent,
            page_size: self.page_size,
            page_token: self.page_token,
        })
    }
}
