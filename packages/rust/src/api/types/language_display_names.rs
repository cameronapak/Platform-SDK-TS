pub use crate::prelude::*;

/// A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageDisplayNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
}

impl LanguageDisplayNames {
    pub fn builder() -> LanguageDisplayNamesBuilder {
        <LanguageDisplayNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageDisplayNamesBuilder {
    en: Option<String>,
}

impl LanguageDisplayNamesBuilder {
    pub fn en(mut self, value: impl Into<String>) -> Self {
        self.en = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguageDisplayNames`].
    pub fn build(self) -> Result<LanguageDisplayNames, BuildError> {
        Ok(LanguageDisplayNames { en: self.en })
    }
}
