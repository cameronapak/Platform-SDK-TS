pub use crate::prelude::*;

/// An unpaginated set of topics related to a query, for pivoting to other verses in the same topic. Backed by Core Search /topics, which returns a fixed set of topics; there is no page_size or page_token. Query metadata (did_you_mean, search_instead_for) is always included.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchTopics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<SearchTopicsTopicsItem>>,
    /// Alternative spellings the search service suggests for the query.
    #[serde(default)]
    pub did_you_mean: Vec<String>,
    /// A corrected query the topics were actually returned for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_instead_for: Option<String>,
    /// Total number of topics returned in this response. Because the endpoint is unpaginated, this is the full count returned, not a running total across pages.
    #[serde(default)]
    pub total_size: i64,
}

impl SearchTopics {
    pub fn builder() -> SearchTopicsBuilder {
        <SearchTopicsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchTopicsBuilder {
    topics: Option<Vec<SearchTopicsTopicsItem>>,
    did_you_mean: Option<Vec<String>>,
    search_instead_for: Option<String>,
    total_size: Option<i64>,
}

impl SearchTopicsBuilder {
    pub fn topics(mut self, value: Vec<SearchTopicsTopicsItem>) -> Self {
        self.topics = Some(value);
        self
    }

    pub fn did_you_mean(mut self, value: Vec<String>) -> Self {
        self.did_you_mean = Some(value);
        self
    }

    pub fn search_instead_for(mut self, value: impl Into<String>) -> Self {
        self.search_instead_for = Some(value.into());
        self
    }

    pub fn total_size(mut self, value: i64) -> Self {
        self.total_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchTopics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`did_you_mean`](SearchTopicsBuilder::did_you_mean)
    /// - [`total_size`](SearchTopicsBuilder::total_size)
    pub fn build(self) -> Result<SearchTopics, BuildError> {
        Ok(SearchTopics {
            topics: self.topics,
            did_you_mean: self
                .did_you_mean
                .ok_or_else(|| BuildError::missing_field("did_you_mean"))?,
            search_instead_for: self.search_instead_for,
            total_size: self
                .total_size
                .ok_or_else(|| BuildError::missing_field("total_size"))?,
        })
    }
}
