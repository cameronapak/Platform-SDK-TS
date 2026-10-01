pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Passage {
    /// A canonical representation of the passage returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Bible text of the requested passage in either text or html format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// A human-readable reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl Passage {
    pub fn builder() -> PassageBuilder {
        <PassageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PassageBuilder {
    id: Option<String>,
    content: Option<String>,
    reference: Option<String>,
}

impl PassageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Passage`].
    pub fn build(self) -> Result<Passage, BuildError> {
        Ok(Passage {
            id: self.id,
            content: self.content,
            reference: self.reference,
        })
    }
}
