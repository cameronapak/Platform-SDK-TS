pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataExchangeTokenCreate {
    /// Data exchange permissions the user should review in the browser flow.
    #[serde(default)]
    pub requested_permissions: Vec<String>,
}

impl DataExchangeTokenCreate {
    pub fn builder() -> DataExchangeTokenCreateBuilder {
        <DataExchangeTokenCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataExchangeTokenCreateBuilder {
    requested_permissions: Option<Vec<String>>,
}

impl DataExchangeTokenCreateBuilder {
    pub fn requested_permissions(mut self, value: Vec<String>) -> Self {
        self.requested_permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataExchangeTokenCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested_permissions`](DataExchangeTokenCreateBuilder::requested_permissions)
    pub fn build(self) -> Result<DataExchangeTokenCreate, BuildError> {
        Ok(DataExchangeTokenCreate {
            requested_permissions: self
                .requested_permissions
                .ok_or_else(|| BuildError::missing_field("requested_permissions"))?,
        })
    }
}
