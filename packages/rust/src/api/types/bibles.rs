pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Bibles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BiblesDataItem>>,
    /// Token to send to server when retrieving the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Total number of bibles in collection matching parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}

impl Bibles {
    pub fn builder() -> BiblesBuilder {
        <BiblesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BiblesBuilder {
    data: Option<Vec<BiblesDataItem>>,
    next_page_token: Option<String>,
    total_size: Option<i64>,
}

impl BiblesBuilder {
    pub fn data(mut self, value: Vec<BiblesDataItem>) -> Self {
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

    /// Consumes the builder and constructs a [`Bibles`].
    pub fn build(self) -> Result<Bibles, BuildError> {
        Ok(Bibles {
            data: self.data,
            next_page_token: self.next_page_token,
            total_size: self.total_size,
        })
    }
}
