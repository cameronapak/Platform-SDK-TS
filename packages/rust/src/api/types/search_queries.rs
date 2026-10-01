pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SearchQueriesDataItem>>,
}

impl SearchQueries {
    pub fn builder() -> SearchQueriesBuilder {
        <SearchQueriesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueriesBuilder {
    data: Option<Vec<SearchQueriesDataItem>>,
}

impl SearchQueriesBuilder {
    pub fn data(mut self, value: Vec<SearchQueriesDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchQueries`].
    pub fn build(self) -> Result<SearchQueries, BuildError> {
        Ok(SearchQueries { data: self.data })
    }
}
