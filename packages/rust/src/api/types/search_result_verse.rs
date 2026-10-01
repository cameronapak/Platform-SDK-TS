pub use crate::prelude::*;

/// A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchResultVerse {
    /// USFM scripture reference for the matched verse.
    #[serde(default)]
    pub reference: String,
}

impl SearchResultVerse {
    pub fn builder() -> SearchResultVerseBuilder {
        <SearchResultVerseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResultVerseBuilder {
    reference: Option<String>,
}

impl SearchResultVerseBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchResultVerse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](SearchResultVerseBuilder::reference)
    pub fn build(self) -> Result<SearchResultVerse, BuildError> {
        Ok(SearchResultVerse {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
