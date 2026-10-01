pub use crate::prelude::*;

/// Query parameters for v1LanguagesCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LanguagesCollectionGetQueryRequest {
    /// The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<V1LanguagesCollectionGetRequestPageSize>,
    /// A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    #[serde(rename = "fields[]")]
    #[serde(default)]
    pub fields_array: Vec<Option<String>>,
    /// The page token to retrieve results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// The ISO 3166 2 character country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Filter languages based on whether Bible content is available for that language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bibles_available: Option<bool>,
}

impl V1LanguagesCollectionGetQueryRequest {
    pub fn builder() -> V1LanguagesCollectionGetQueryRequestBuilder {
        <V1LanguagesCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LanguagesCollectionGetQueryRequestBuilder {
    page_size: Option<V1LanguagesCollectionGetRequestPageSize>,
    fields_array: Option<Vec<Option<String>>>,
    page_token: Option<String>,
    country: Option<String>,
    bibles_available: Option<bool>,
}

impl V1LanguagesCollectionGetQueryRequestBuilder {
    pub fn page_size(mut self, value: V1LanguagesCollectionGetRequestPageSize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn fields_array(mut self, value: Vec<Option<String>>) -> Self {
        self.fields_array = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn bibles_available(mut self, value: bool) -> Self {
        self.bibles_available = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1LanguagesCollectionGetQueryRequest`].
    pub fn build(self) -> Result<V1LanguagesCollectionGetQueryRequest, BuildError> {
        Ok(V1LanguagesCollectionGetQueryRequest {
            page_size: self.page_size,
            fields_array: self.fields_array.unwrap_or_default(),
            page_token: self.page_token,
            country: self.country,
            bibles_available: self.bibles_available,
        })
    }
}
