pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1OrganizationsBiblesCollectionGetResponseDataItem {
    /// Bible version identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Bible version abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    /// Longer form of copyright text provided by the publisher for the given Bible version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_content: Option<String>,
    /// Short version of the copyright text provided by the publisher for the given Bible version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// Additional information about the Bible text version. This is commonly displayed in the reader footer for the Bible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// URL to link to publisher page from the reader's footer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_url: Option<String>,
    /// BCP47 canonical language tag for this Bible version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_tag: Option<String>,
    /// Localized Bible version abbreviation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_abbreviation: Option<String>,
    /// Localized title of Bible version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_title: Option<String>,
    /// English title of Bible version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub books: Option<Vec<String>>,
    /// A deep link to this Bible version inside YouVersion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youversion_deep_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

impl V1OrganizationsBiblesCollectionGetResponseDataItem {
    pub fn builder() -> V1OrganizationsBiblesCollectionGetResponseDataItemBuilder {
        <V1OrganizationsBiblesCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsBiblesCollectionGetResponseDataItemBuilder {
    id: Option<i64>,
    abbreviation: Option<String>,
    promotional_content: Option<String>,
    copyright: Option<String>,
    info: Option<String>,
    publisher_url: Option<String>,
    language_tag: Option<String>,
    localized_abbreviation: Option<String>,
    localized_title: Option<String>,
    title: Option<String>,
    books: Option<Vec<String>>,
    youversion_deep_link: Option<String>,
    organization_id: Option<String>,
}

impl V1OrganizationsBiblesCollectionGetResponseDataItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn abbreviation(mut self, value: impl Into<String>) -> Self {
        self.abbreviation = Some(value.into());
        self
    }

    pub fn promotional_content(mut self, value: impl Into<String>) -> Self {
        self.promotional_content = Some(value.into());
        self
    }

    pub fn copyright(mut self, value: impl Into<String>) -> Self {
        self.copyright = Some(value.into());
        self
    }

    pub fn info(mut self, value: impl Into<String>) -> Self {
        self.info = Some(value.into());
        self
    }

    pub fn publisher_url(mut self, value: impl Into<String>) -> Self {
        self.publisher_url = Some(value.into());
        self
    }

    pub fn language_tag(mut self, value: impl Into<String>) -> Self {
        self.language_tag = Some(value.into());
        self
    }

    pub fn localized_abbreviation(mut self, value: impl Into<String>) -> Self {
        self.localized_abbreviation = Some(value.into());
        self
    }

    pub fn localized_title(mut self, value: impl Into<String>) -> Self {
        self.localized_title = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn books(mut self, value: Vec<String>) -> Self {
        self.books = Some(value);
        self
    }

    pub fn youversion_deep_link(mut self, value: impl Into<String>) -> Self {
        self.youversion_deep_link = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsBiblesCollectionGetResponseDataItem`].
    pub fn build(self) -> Result<V1OrganizationsBiblesCollectionGetResponseDataItem, BuildError> {
        Ok(V1OrganizationsBiblesCollectionGetResponseDataItem {
            id: self.id,
            abbreviation: self.abbreviation,
            promotional_content: self.promotional_content,
            copyright: self.copyright,
            info: self.info,
            publisher_url: self.publisher_url,
            language_tag: self.language_tag,
            localized_abbreviation: self.localized_abbreviation,
            localized_title: self.localized_title,
            title: self.title,
            books: self.books,
            youversion_deep_link: self.youversion_deep_link,
            organization_id: self.organization_id,
        })
    }
}
