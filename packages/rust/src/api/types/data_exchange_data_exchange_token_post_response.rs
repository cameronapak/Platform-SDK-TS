pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DataExchangeTokenPostResponse {
    /// Opaque single-use token for the data exchange flow. Returned only once.
    #[serde(default)]
    pub token: String,
    pub token_type: String,
    /// Token lifetime in seconds.
    #[serde(default)]
    pub expires_in: i64,
}

impl DataExchangeTokenPostResponse {
    pub fn builder() -> DataExchangeTokenPostResponseBuilder {
        <DataExchangeTokenPostResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataExchangeTokenPostResponseBuilder {
    token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<i64>,
}

impl DataExchangeTokenPostResponseBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn token_type(mut self, value: impl Into<String>) -> Self {
        self.token_type = Some(value.into());
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataExchangeTokenPostResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](DataExchangeTokenPostResponseBuilder::token)
    /// - [`token_type`](DataExchangeTokenPostResponseBuilder::token_type)
    /// - [`expires_in`](DataExchangeTokenPostResponseBuilder::expires_in)
    pub fn build(self) -> Result<DataExchangeTokenPostResponse, BuildError> {
        Ok(DataExchangeTokenPostResponse {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            token_type: self
                .token_type
                .ok_or_else(|| BuildError::missing_field("token_type"))?,
            expires_in: self
                .expires_in
                .ok_or_else(|| BuildError::missing_field("expires_in"))?,
        })
    }
}
