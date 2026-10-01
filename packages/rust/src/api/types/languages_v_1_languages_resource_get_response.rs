pub use crate::prelude::*;

/// Region-agnostic language resource keyed by canonical BCP 47 language or language+script. Variants and a simple extensions indicator are included for completeness, but regions are excluded except for a small, enumerated set of region variants that speakers treat as distinct languages (for example pt-PT vs pt-BR, es-ES vs es-419); full extension data is always excluded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LanguagesResourceGetResponse {
    /// Canonical BCP 47 id limited to language, language+script, or one of the enumerated region-significant variants (es-419, es-ES, pt-BR, pt-PT, zh-Hant-HK, zh-Hant-TW). Other regions, variants, and extensions are not allowed.
    #[serde(default)]
    pub id: String,
    /// ISO 639 canonical language subtag
    #[serde(default)]
    pub language: String,
    /// ISO 15924 script code if present in id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    /// The English name for the script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    /// Deprecated or legacy subtags mapped during canonicalization for this language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// A map of every known display name for this language, keyed by the locale the name is written in. Covers all locales the platform can render (e.g. the English, endonym, and hundreds of other localized names), so this object is large; request fields[] without display_names to omit it when you only need localized_name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_names: Option<V1LanguagesResourceGetResponseDisplayNames>,
    /// The single display name chosen from display_names for the request's Accept-Language header. Selection is per language and uses the Accept-Language priority list, then falls back to the English name, then the first available name. The Content-Language response header reports the locale negotiated for the response as a whole; in a collection an individual language may fall back to a different locale than that header when it has no name in the negotiated one. Use this when you want one name to show the user; use display_names when you need the full set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_name: Option<String>,
    /// All scripts known for this language (CLDR/ISO-15924)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
    /// Variants associated with this language (not part of the id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<String>>,
    /// Ids of countries where this language is used or supported. Extended details can be retrieved from the countries API with the provided id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    /// Default text direction for this language. ltr is left to right and rtl is right to left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<V1LanguagesResourceGetResponseTextDirection>,
    /// Estimated number of souls that write in this language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writing_population: Option<i64>,
    /// Estimated number of souls that speak in this language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaking_population: Option<i64>,
    /// The chosen default Bible version for this language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bible_id: Option<i64>,
}

impl V1LanguagesResourceGetResponse {
    pub fn builder() -> V1LanguagesResourceGetResponseBuilder {
        <V1LanguagesResourceGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LanguagesResourceGetResponseBuilder {
    id: Option<String>,
    language: Option<String>,
    script: Option<String>,
    script_name: Option<String>,
    aliases: Option<Vec<String>>,
    display_names: Option<V1LanguagesResourceGetResponseDisplayNames>,
    localized_name: Option<String>,
    scripts: Option<Vec<String>>,
    variants: Option<Vec<String>>,
    countries: Option<Vec<String>>,
    text_direction: Option<V1LanguagesResourceGetResponseTextDirection>,
    writing_population: Option<i64>,
    speaking_population: Option<i64>,
    default_bible_id: Option<i64>,
}

impl V1LanguagesResourceGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn script(mut self, value: impl Into<String>) -> Self {
        self.script = Some(value.into());
        self
    }

    pub fn script_name(mut self, value: impl Into<String>) -> Self {
        self.script_name = Some(value.into());
        self
    }

    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn display_names(mut self, value: V1LanguagesResourceGetResponseDisplayNames) -> Self {
        self.display_names = Some(value);
        self
    }

    pub fn localized_name(mut self, value: impl Into<String>) -> Self {
        self.localized_name = Some(value.into());
        self
    }

    pub fn scripts(mut self, value: Vec<String>) -> Self {
        self.scripts = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<String>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn text_direction(mut self, value: V1LanguagesResourceGetResponseTextDirection) -> Self {
        self.text_direction = Some(value);
        self
    }

    pub fn writing_population(mut self, value: i64) -> Self {
        self.writing_population = Some(value);
        self
    }

    pub fn speaking_population(mut self, value: i64) -> Self {
        self.speaking_population = Some(value);
        self
    }

    pub fn default_bible_id(mut self, value: i64) -> Self {
        self.default_bible_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1LanguagesResourceGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](V1LanguagesResourceGetResponseBuilder::id)
    /// - [`language`](V1LanguagesResourceGetResponseBuilder::language)
    pub fn build(self) -> Result<V1LanguagesResourceGetResponse, BuildError> {
        Ok(V1LanguagesResourceGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            script: self.script,
            script_name: self.script_name,
            aliases: self.aliases,
            display_names: self.display_names,
            localized_name: self.localized_name,
            scripts: self.scripts,
            variants: self.variants,
            countries: self.countries,
            text_direction: self.text_direction,
            writing_population: self.writing_population,
            speaking_population: self.speaking_population,
            default_bible_id: self.default_bible_id,
        })
    }
}
