pub use crate::prelude::*;

/// A public app summary resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppSummariesDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AppSummariesDataItemStatus>,
}

impl AppSummariesDataItem {
    pub fn builder() -> AppSummariesDataItemBuilder {
        <AppSummariesDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppSummariesDataItemBuilder {
    app_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    website_url: Option<String>,
    status: Option<AppSummariesDataItemStatus>,
}

impl AppSummariesDataItemBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppSummariesDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppSummariesDataItem`].
    pub fn build(self) -> Result<AppSummariesDataItem, BuildError> {
        Ok(AppSummariesDataItem {
            app_id: self.app_id,
            name: self.name,
            description: self.description,
            website_url: self.website_url,
            status: self.status,
        })
    }
}
