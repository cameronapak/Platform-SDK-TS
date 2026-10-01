pub use crate::prelude::*;

/// Query parameters for collectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CollectionGetQueryRequest {
    /// This parameter is used on some collections to modify the resources returned. For example, it modifies whether all Bibles in the Platform should be included in the Bibles collection regardless of licensing of the provided app key. It modified the Licenses collection so that the response will include every license, regardless of whether the developer has agreed to it yet. The default for this field in all cases is false. If a developer wants to include all resources for a collection that implements this query parameter, the client must specifically pass it as true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_available: Option<bool>,
    /// An ordered list of language ranges using bracket notation. Use repeated parameters like `language_ranges[]=en&language_ranges[]=es` to supply multiple ranges, and use the wildcard * to match all languages. A language range is much like a language tag but may contain wildcards.  See RFC 4647 section 2 for the full definition: https://www.rfc-editor.org/rfc/rfc4647.html#section-2
    /// Language ranges in this parameter may only be of the Basic Range format.
    #[serde(rename = "language_ranges[]")]
    #[serde(default)]
    pub language_ranges_array: Vec<Option<String>>,
    /// Filter Bibles by a license identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_id: Option<i64>,
    /// The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<BiblesCollectionGetRequestPageSize>,
    /// A list of top-level fields to include in each resource object. Use bracket notation to pass
    /// multiple values, for example: `fields[]=id&fields[]=name&fields[]=language`.
    /// When provided, `page_size=*` is allowed only if the number of fields requested is three (3) or fewer.
    #[serde(rename = "fields[]")]
    #[serde(default)]
    pub fields_array: Vec<Option<String>>,
    /// The page token to retrieve results from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl CollectionGetQueryRequest {
    pub fn builder() -> CollectionGetQueryRequestBuilder {
        <CollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CollectionGetQueryRequestBuilder {
    all_available: Option<bool>,
    language_ranges_array: Option<Vec<Option<String>>>,
    license_id: Option<i64>,
    page_size: Option<BiblesCollectionGetRequestPageSize>,
    fields_array: Option<Vec<Option<String>>>,
    page_token: Option<String>,
}

impl CollectionGetQueryRequestBuilder {
    pub fn all_available(mut self, value: bool) -> Self {
        self.all_available = Some(value);
        self
    }

    pub fn language_ranges_array(mut self, value: Vec<Option<String>>) -> Self {
        self.language_ranges_array = Some(value);
        self
    }

    pub fn license_id(mut self, value: i64) -> Self {
        self.license_id = Some(value);
        self
    }

    pub fn page_size(mut self, value: BiblesCollectionGetRequestPageSize) -> Self {
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

    /// Consumes the builder and constructs a [`CollectionGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_ranges_array`](CollectionGetQueryRequestBuilder::language_ranges_array)
    pub fn build(self) -> Result<CollectionGetQueryRequest, BuildError> {
        Ok(CollectionGetQueryRequest {
            all_available: self.all_available,
            language_ranges_array: self
                .language_ranges_array
                .ok_or_else(|| BuildError::missing_field("language_ranges_array"))?,
            license_id: self.license_id,
            page_size: self.page_size,
            fields_array: self.fields_array.unwrap_or_default(),
            page_token: self.page_token,
        })
    }
}
