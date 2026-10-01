pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotFoundErrorBody {
    /// Error message
    #[serde(default)]
    pub message: String,
}

impl NotFoundErrorBody {
    pub fn builder() -> NotFoundErrorBodyBuilder {
        <NotFoundErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotFoundErrorBodyBuilder {
    message: Option<String>,
}

impl NotFoundErrorBodyBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotFoundErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](NotFoundErrorBodyBuilder::message)
    pub fn build(self) -> Result<NotFoundErrorBody, BuildError> {
        Ok(NotFoundErrorBody {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
