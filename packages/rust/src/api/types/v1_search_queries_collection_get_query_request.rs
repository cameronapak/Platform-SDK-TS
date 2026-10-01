pub use crate::prelude::*;

/// Query parameters for v1SearchQueriesCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchQueriesCollectionGetQueryRequest {
    /// An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    #[serde(rename = "language_ranges[]")]
    #[serde(default)]
    pub language_ranges_array: Vec<Option<String>>,
    /// The partial query for as-you-type suggestions. Omit for trending queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return recently popular searches for the language instead of suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trending: Option<bool>,
}

impl V1SearchQueriesCollectionGetQueryRequest {
    pub fn builder() -> V1SearchQueriesCollectionGetQueryRequestBuilder {
        <V1SearchQueriesCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchQueriesCollectionGetQueryRequestBuilder {
    language_ranges_array: Option<Vec<Option<String>>>,
    query: Option<String>,
    trending: Option<bool>,
}

impl V1SearchQueriesCollectionGetQueryRequestBuilder {
    pub fn language_ranges_array(mut self, value: Vec<Option<String>>) -> Self {
        self.language_ranges_array = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn trending(mut self, value: bool) -> Self {
        self.trending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1SearchQueriesCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_ranges_array`](V1SearchQueriesCollectionGetQueryRequestBuilder::language_ranges_array)
    pub fn build(self) -> Result<V1SearchQueriesCollectionGetQueryRequest, BuildError> {
        Ok(V1SearchQueriesCollectionGetQueryRequest {
            language_ranges_array: self
                .language_ranges_array
                .ok_or_else(|| BuildError::missing_field("language_ranges_array"))?,
            query: self.query,
            trending: self.trending,
        })
    }
}
