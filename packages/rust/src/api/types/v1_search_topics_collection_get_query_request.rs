pub use crate::prelude::*;

/// Query parameters for v1SearchTopicsCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchTopicsCollectionGetQueryRequest {
    /// The search query string used to find matching results.
    #[serde(default)]
    pub query: String,
    /// An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    #[serde(rename = "language_ranges[]")]
    #[serde(default)]
    pub language_ranges_array: Vec<Option<String>>,
}

impl V1SearchTopicsCollectionGetQueryRequest {
    pub fn builder() -> V1SearchTopicsCollectionGetQueryRequestBuilder {
        <V1SearchTopicsCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchTopicsCollectionGetQueryRequestBuilder {
    query: Option<String>,
    language_ranges_array: Option<Vec<Option<String>>>,
}

impl V1SearchTopicsCollectionGetQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn language_ranges_array(mut self, value: Vec<Option<String>>) -> Self {
        self.language_ranges_array = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1SearchTopicsCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](V1SearchTopicsCollectionGetQueryRequestBuilder::query)
    /// - [`language_ranges_array`](V1SearchTopicsCollectionGetQueryRequestBuilder::language_ranges_array)
    pub fn build(self) -> Result<V1SearchTopicsCollectionGetQueryRequest, BuildError> {
        Ok(V1SearchTopicsCollectionGetQueryRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            language_ranges_array: self
                .language_ranges_array
                .ok_or_else(|| BuildError::missing_field("language_ranges_array"))?,
        })
    }
}
