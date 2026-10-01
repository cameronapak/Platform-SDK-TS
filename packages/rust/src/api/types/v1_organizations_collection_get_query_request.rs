pub use crate::prelude::*;

/// Query parameters for v1OrganizationsCollectionGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1OrganizationsCollectionGetQueryRequest {
    /// Filter organizations to those associated with the given Bible version(s). Use bracket notation: bible_ids[]=111&bible_ids[]=206. When omitted, returns all organizations.
    #[serde(rename = "bible_ids[]")]
    #[serde(default)]
    pub bible_ids_array: Vec<Option<i64>>,
    /// The number of items to return in the collection.  Numeric values must be between 1 and 99.
    /// Special value "*" is supported only when used in combination with the `fields` parameter and
    /// when the client requests three or fewer fields (see `fields` parameter). When "*" is used the
    /// server will return all matching items for the requested resource (no numeric page limit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<V1OrganizationsCollectionGetRequestPageSize>,
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

impl V1OrganizationsCollectionGetQueryRequest {
    pub fn builder() -> V1OrganizationsCollectionGetQueryRequestBuilder {
        <V1OrganizationsCollectionGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsCollectionGetQueryRequestBuilder {
    bible_ids_array: Option<Vec<Option<i64>>>,
    page_size: Option<V1OrganizationsCollectionGetRequestPageSize>,
    fields_array: Option<Vec<Option<String>>>,
    page_token: Option<String>,
}

impl V1OrganizationsCollectionGetQueryRequestBuilder {
    pub fn bible_ids_array(mut self, value: Vec<Option<i64>>) -> Self {
        self.bible_ids_array = Some(value);
        self
    }

    pub fn page_size(mut self, value: V1OrganizationsCollectionGetRequestPageSize) -> Self {
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

    /// Consumes the builder and constructs a [`V1OrganizationsCollectionGetQueryRequest`].
    pub fn build(self) -> Result<V1OrganizationsCollectionGetQueryRequest, BuildError> {
        Ok(V1OrganizationsCollectionGetQueryRequest {
            bible_ids_array: self.bible_ids_array.unwrap_or_default(),
            page_size: self.page_size,
            fields_array: self.fields_array.unwrap_or_default(),
            page_token: self.page_token,
        })
    }
}
