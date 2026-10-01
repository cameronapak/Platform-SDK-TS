pub use crate::prelude::*;

/// A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchVersesCollectionGetResponseVersesItem {
    /// USFM scripture reference for the matched verse.
    #[serde(default)]
    pub reference: String,
}

impl V1SearchVersesCollectionGetResponseVersesItem {
    pub fn builder() -> V1SearchVersesCollectionGetResponseVersesItemBuilder {
        <V1SearchVersesCollectionGetResponseVersesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchVersesCollectionGetResponseVersesItemBuilder {
    reference: Option<String>,
}

impl V1SearchVersesCollectionGetResponseVersesItemBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1SearchVersesCollectionGetResponseVersesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](V1SearchVersesCollectionGetResponseVersesItemBuilder::reference)
    pub fn build(self) -> Result<V1SearchVersesCollectionGetResponseVersesItem, BuildError> {
        Ok(V1SearchVersesCollectionGetResponseVersesItem {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
