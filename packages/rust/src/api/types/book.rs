pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Book {
    /// Book identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Book title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Full book title if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_title: Option<String>,
    /// Book name abbreviation if provided by the publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    /// Canon identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canon: Option<BookCanon>,
    /// Chapters in the book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<BookChaptersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<BookIntro>,
}

impl Book {
    pub fn builder() -> BookBuilder {
        <BookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BookBuilder {
    id: Option<String>,
    title: Option<String>,
    full_title: Option<String>,
    abbreviation: Option<String>,
    canon: Option<BookCanon>,
    chapters: Option<Vec<BookChaptersItem>>,
    intro: Option<BookIntro>,
}

impl BookBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn full_title(mut self, value: impl Into<String>) -> Self {
        self.full_title = Some(value.into());
        self
    }

    pub fn abbreviation(mut self, value: impl Into<String>) -> Self {
        self.abbreviation = Some(value.into());
        self
    }

    pub fn canon(mut self, value: BookCanon) -> Self {
        self.canon = Some(value);
        self
    }

    pub fn chapters(mut self, value: Vec<BookChaptersItem>) -> Self {
        self.chapters = Some(value);
        self
    }

    pub fn intro(mut self, value: BookIntro) -> Self {
        self.intro = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Book`].
    pub fn build(self) -> Result<Book, BuildError> {
        Ok(Book {
            id: self.id,
            title: self.title,
            full_title: self.full_title,
            abbreviation: self.abbreviation,
            canon: self.canon,
            chapters: self.chapters,
            intro: self.intro,
        })
    }
}
