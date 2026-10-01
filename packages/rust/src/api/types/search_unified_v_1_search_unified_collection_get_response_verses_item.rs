pub use crate::prelude::*;

/// A verse result: a scripture reference and metadata only, never passage text. Resolve verse text through the licensed-content endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1SearchUnifiedCollectionGetResponseVersesItem {
    /// USFM scripture reference for the matched verse.
    #[serde(default)]
    pub reference: String,
}

impl V1SearchUnifiedCollectionGetResponseVersesItem {
    pub fn builder() -> V1SearchUnifiedCollectionGetResponseVersesItemBuilder {
        <V1SearchUnifiedCollectionGetResponseVersesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1SearchUnifiedCollectionGetResponseVersesItemBuilder {
    reference: Option<String>,
}

impl V1SearchUnifiedCollectionGetResponseVersesItemBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1SearchUnifiedCollectionGetResponseVersesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](V1SearchUnifiedCollectionGetResponseVersesItemBuilder::reference)
    pub fn build(self) -> Result<V1SearchUnifiedCollectionGetResponseVersesItem, BuildError> {
        Ok(V1SearchUnifiedCollectionGetResponseVersesItem {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}
