pub use crate::prelude::*;

/// Query parameters for v1SearchUnifiedCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchUnifiedCollectionGetQueryRequest {
    /// The search query string used to find matching results.
    #[serde(default)]
    pub query: String,
    /// The Bible version identifier
    #[serde(default)]
    pub bible_id: i64,
    /// An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    #[serde(rename = "language_ranges[]")]
    #[serde(default)]
    pub language_ranges_array: Vec<Option<String>>,
    /// The searcher's intent. Defaults to unknown, matching the Core Search service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_intent: Option<V1SearchUnifiedCollectionGetRequestUserIntent>,
    /// Result kinds to include: `verses` and/or `topics`. Use bracket notation to pass multiple
    /// values, for example `fields[]=verses&fields[]=topics`. Omit to return all kinds. Query
    /// metadata is always included.
    #[serde(rename = "fields[]")]
    #[serde(default)]
    pub fields_array: Vec<Option<String>>,
}

impl V1SearchUnifiedCollectionGetQueryRequest {
    pub fn builder() -> V1SearchUnifiedCollectionGetQueryRequestBuilder {
        <V1SearchUnifiedCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchUnifiedCollectionGetQueryRequestBuilder {
    query: Option<String>,
    bible_id: Option<i64>,
    language_ranges_array: Option<Vec<Option<String>>>,
    user_intent: Option<V1SearchUnifiedCollectionGetRequestUserIntent>,
    fields_array: Option<Vec<Option<String>>>,
}

impl V1SearchUnifiedCollectionGetQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn bible_id(mut self, value: i64) -> Self {
        self.bible_id = Some(value);
        self
    }

    pub fn language_ranges_array(mut self, value: Vec<Option<String>>) -> Self {
        self.language_ranges_array = Some(value);
        self
    }

    pub fn user_intent(mut self, value: V1SearchUnifiedCollectionGetRequestUserIntent) -> Self {
        self.user_intent = Some(value);
        self
    }

    pub fn fields_array(mut self, value: Vec<Option<String>>) -> Self {
        self.fields_array = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1SearchUnifiedCollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](V1SearchUnifiedCollectionGetQueryRequestBuilder::query)
    /// - [`bible_id`](V1SearchUnifiedCollectionGetQueryRequestBuilder::bible_id)
    /// - [`language_ranges_array`](V1SearchUnifiedCollectionGetQueryRequestBuilder::language_ranges_array)
    pub fn build(self) -> Result<V1SearchUnifiedCollectionGetQueryRequest, BuildError> {
        Ok(V1SearchUnifiedCollectionGetQueryRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            bible_id: self
                .bible_id
                .ok_or_else(|| BuildError::missing_field("bible_id"))?,
            language_ranges_array: self
                .language_ranges_array
                .ok_or_else(|| BuildError::missing_field("language_ranges_array"))?,
            user_intent: self.user_intent,
            fields_array: self.fields_array.unwrap_or_default(),
        })
    }
}
