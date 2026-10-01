pub use crate::prelude::*;

/// A query object: a search string a user might run, plus where it came from (e.g. a community suggestion or a trending search).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueriesDataItem {
    /// The suggested or trending search query string.
    #[serde(default)]
    pub text: String,
    /// Where the query came from, e.g. `community` or `trending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl SearchQueriesDataItem {
    pub fn builder() -> SearchQueriesDataItemBuilder {
        <SearchQueriesDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueriesDataItemBuilder {
    text: Option<String>,
    source: Option<String>,
}

impl SearchQueriesDataItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchQueriesDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SearchQueriesDataItemBuilder::text)
    pub fn build(self) -> Result<SearchQueriesDataItem, BuildError> {
        Ok(SearchQueriesDataItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            source: self.source,
        })
    }
}
