pub use crate::prelude::*;

/// A collection of public app summary resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppSummaries {
    #[serde(default)]
    pub data: Vec<AppSummariesDataItem>,
}

impl AppSummaries {
    pub fn builder() -> AppSummariesBuilder {
        <AppSummariesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppSummariesBuilder {
    data: Option<Vec<AppSummariesDataItem>>,
}

impl AppSummariesBuilder {
    pub fn data(mut self, value: Vec<AppSummariesDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppSummaries`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AppSummariesBuilder::data)
    pub fn build(self) -> Result<AppSummaries, BuildError> {
        Ok(AppSummaries {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
