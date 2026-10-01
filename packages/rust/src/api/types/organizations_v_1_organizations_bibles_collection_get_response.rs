pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1OrganizationsBiblesCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1OrganizationsBiblesCollectionGetResponseDataItem>>,
    /// Token to send to server when retrieving the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Total number of bibles in collection matching parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}

impl V1OrganizationsBiblesCollectionGetResponse {
    pub fn builder() -> V1OrganizationsBiblesCollectionGetResponseBuilder {
        <V1OrganizationsBiblesCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsBiblesCollectionGetResponseBuilder {
    data: Option<Vec<V1OrganizationsBiblesCollectionGetResponseDataItem>>,
    next_page_token: Option<String>,
    total_size: Option<i64>,
}

impl V1OrganizationsBiblesCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1OrganizationsBiblesCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn total_size(mut self, value: i64) -> Self {
        self.total_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsBiblesCollectionGetResponse`].
    pub fn build(self) -> Result<V1OrganizationsBiblesCollectionGetResponse, BuildError> {
        Ok(V1OrganizationsBiblesCollectionGetResponse {
            data: self.data,
            next_page_token: self.next_page_token,
            total_size: self.total_size,
        })
    }
}
