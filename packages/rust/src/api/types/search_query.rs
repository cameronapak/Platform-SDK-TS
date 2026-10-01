pub use crate::prelude::*;

/// A query object: a search string a user might run, plus where it came from (e.g. a community suggestion or a trending search).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQuery {
    /// The suggested or trending search query string.
    #[serde(default)]
    pub text: String,
    /// Where the query came from, e.g. `community` or `trending`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl SearchQuery {
    pub fn builder() -> SearchQueryBuilder {
        <SearchQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueryBuilder {
    text: Option<String>,
    source: Option<String>,
}

impl SearchQueryBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchQuery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SearchQueryBuilder::text)
    pub fn build(self) -> Result<SearchQuery, BuildError> {
        Ok(SearchQuery {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            source: self.source,
        })
    }
}
