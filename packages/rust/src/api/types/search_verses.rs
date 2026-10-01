pub use crate::prelude::*;

/// A paginated list of Bible verse search results. Verse results carry references and metadata only (e.g. JHN.3.16), never passage text; resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchVerses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verses: Option<Vec<SearchVersesVersesItem>>,
    /// The intent the Core Search service resolved for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_intent: Option<String>,
    /// Alternative spellings the search service suggests for the query.
    #[serde(default)]
    pub did_you_mean: Vec<String>,
    /// A corrected query the results were actually returned for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_instead_for: Option<String>,
    /// Token to send to the server when retrieving the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl SearchVerses {
    pub fn builder() -> SearchVersesBuilder {
        <SearchVersesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchVersesBuilder {
    verses: Option<Vec<SearchVersesVersesItem>>,
    user_intent: Option<String>,
    did_you_mean: Option<Vec<String>>,
    search_instead_for: Option<String>,
    next_page_token: Option<String>,
}

impl SearchVersesBuilder {
    pub fn verses(mut self, value: Vec<SearchVersesVersesItem>) -> Self {
        self.verses = Some(value);
        self
    }

    pub fn user_intent(mut self, value: impl Into<String>) -> Self {
        self.user_intent = Some(value.into());
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

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchVerses`].
    /// This method will fail if any of the following fields are not set:
    /// - [`did_you_mean`](SearchVersesBuilder::did_you_mean)
    pub fn build(self) -> Result<SearchVerses, BuildError> {
        Ok(SearchVerses {
            verses: self.verses,
            user_intent: self.user_intent,
            did_you_mean: self
                .did_you_mean
                .ok_or_else(|| BuildError::missing_field("did_you_mean"))?,
            search_instead_for: self.search_instead_for,
            next_page_token: self.next_page_token,
        })
    }
}
