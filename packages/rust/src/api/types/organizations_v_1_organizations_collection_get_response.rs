pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct V1OrganizationsCollectionGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<V1OrganizationsCollectionGetResponseDataItem>>,
    /// Token to send to server when retrieving the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl V1OrganizationsCollectionGetResponse {
    pub fn builder() -> V1OrganizationsCollectionGetResponseBuilder {
        <V1OrganizationsCollectionGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsCollectionGetResponseBuilder {
    data: Option<Vec<V1OrganizationsCollectionGetResponseDataItem>>,
    next_page_token: Option<String>,
}

impl V1OrganizationsCollectionGetResponseBuilder {
    pub fn data(mut self, value: Vec<V1OrganizationsCollectionGetResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsCollectionGetResponse`].
    pub fn build(self) -> Result<V1OrganizationsCollectionGetResponse, BuildError> {
        Ok(V1OrganizationsCollectionGetResponse {
            data: self.data,
            next_page_token: self.next_page_token,
        })
    }
}
