pub use crate::prelude::*;

/// Query parameters for passagesResourceGet
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PassagesResourceGetQueryRequest {
    /// The desired Bible content format (text or html). Headings and notes are included only when format=html; format=text returns verse text without them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<BiblesPassagesResourceGetRequestFormat>,
    /// Whether or not headings should be included in the Bible content. Headings are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_headings: Option<bool>,
    /// Whether or not notes should be included in the Bible content. Notes are returned only when format=html; they are omitted when format=text. The default for this field is false unless the reference is a chapter or introduction (GEN.1 or GEN.INTRO1) in which case it would be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_notes: Option<bool>,
}

impl PassagesResourceGetQueryRequest {
    pub fn builder() -> PassagesResourceGetQueryRequestBuilder {
        <PassagesResourceGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PassagesResourceGetQueryRequestBuilder {
    format: Option<BiblesPassagesResourceGetRequestFormat>,
    include_headings: Option<bool>,
    include_notes: Option<bool>,
}

impl PassagesResourceGetQueryRequestBuilder {
    pub fn format(mut self, value: BiblesPassagesResourceGetRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn include_headings(mut self, value: bool) -> Self {
        self.include_headings = Some(value);
        self
    }

    pub fn include_notes(mut self, value: bool) -> Self {
        self.include_notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PassagesResourceGetQueryRequest`].
    pub fn build(self) -> Result<PassagesResourceGetQueryRequest, BuildError> {
        Ok(PassagesResourceGetQueryRequest {
            format: self.format,
            include_headings: self.include_headings,
            include_notes: self.include_notes,
        })
    }
}
