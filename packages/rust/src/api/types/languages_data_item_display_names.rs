pub use crate::prelude::*;

/// A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguagesDataItemDisplayNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
}

impl LanguagesDataItemDisplayNames {
    pub fn builder() -> LanguagesDataItemDisplayNamesBuilder {
        <LanguagesDataItemDisplayNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguagesDataItemDisplayNamesBuilder {
    en: Option<String>,
}

impl LanguagesDataItemDisplayNamesBuilder {
    pub fn en(mut self, value: impl Into<String>) -> Self {
        self.en = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguagesDataItemDisplayNames`].
    pub fn build(self) -> Result<LanguagesDataItemDisplayNames, BuildError> {
        Ok(LanguagesDataItemDisplayNames { en: self.en })
    }
}
