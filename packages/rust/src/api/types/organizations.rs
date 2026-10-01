pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Organizations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrganizationsDataItem>>,
    /// Token to send to server when retrieving the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Organizations {
    pub fn builder() -> OrganizationsBuilder {
        <OrganizationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationsBuilder {
    data: Option<Vec<OrganizationsDataItem>>,
    next_page_token: Option<String>,
}

impl OrganizationsBuilder {
    pub fn data(mut self, value: Vec<OrganizationsDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Organizations`].
    pub fn build(self) -> Result<Organizations, BuildError> {
        Ok(Organizations {
            data: self.data,
            next_page_token: self.next_page_token,
        })
    }
}
