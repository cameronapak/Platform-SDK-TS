pub use crate::prelude::*;

/// A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchVersesVersesItem {
    /// USFM scripture reference for the matched verse.
    #[serde(default)]
    pub reference: String,
}

impl SearchVersesVersesItem {
    pub fn builder() -> SearchVersesVersesItemBuilder {
        <SearchVersesVersesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchVersesVersesItemBuilder {
    reference: Option<String>,
}

impl SearchVersesVersesItemBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchVersesVersesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](SearchVersesVersesItemBuilder::reference)
    pub fn build(self) -> Result<SearchVersesVersesItem, BuildError> {
        Ok(SearchVersesVersesItem {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
