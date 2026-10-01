pub use crate::prelude::*;

/// A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LanguagesCollectionGetResponseDataItemDisplayNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
}

impl V1LanguagesCollectionGetResponseDataItemDisplayNames {
    pub fn builder() -> V1LanguagesCollectionGetResponseDataItemDisplayNamesBuilder {
        <V1LanguagesCollectionGetResponseDataItemDisplayNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LanguagesCollectionGetResponseDataItemDisplayNamesBuilder {
    en: Option<String>,
}

impl V1LanguagesCollectionGetResponseDataItemDisplayNamesBuilder {
    pub fn en(mut self, value: impl Into<String>) -> Self {
        self.en = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1LanguagesCollectionGetResponseDataItemDisplayNames`].
    pub fn build(self) -> Result<V1LanguagesCollectionGetResponseDataItemDisplayNames, BuildError> {
        Ok(V1LanguagesCollectionGetResponseDataItemDisplayNames { en: self.en })
    }
}
