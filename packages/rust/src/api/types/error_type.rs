pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error {
    /// Error message
    #[serde(default)]
    pub message: String,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    message: Option<String>,
}

impl ErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ErrorBuilder::message)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
