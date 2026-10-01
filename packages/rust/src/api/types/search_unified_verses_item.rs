pub use crate::prelude::*;

/// A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchUnifiedVersesItem {
    /// USFM scripture reference for the matched verse.
    #[serde(default)]
    pub reference: String,
}

impl SearchUnifiedVersesItem {
    pub fn builder() -> SearchUnifiedVersesItemBuilder {
        <SearchUnifiedVersesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchUnifiedVersesItemBuilder {
    reference: Option<String>,
}

impl SearchUnifiedVersesItemBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchUnifiedVersesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](SearchUnifiedVersesItemBuilder::reference)
    pub fn build(self) -> Result<SearchUnifiedVersesItem, BuildError> {
        Ok(SearchUnifiedVersesItem {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
