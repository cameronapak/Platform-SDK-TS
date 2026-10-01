pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1LicensesCollectionGetResponseDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// The YouVersion Organization ID that owns this license, or null for licenses not owned by any organization (e.g. Public Domain and Creative Commons).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// HTML representation of the license terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bible_ids: Option<Vec<i64>>,
    /// URI pointing to the license terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The date which the developer id passed agreed to this license or null if not agreed to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub agreed_dt: Option<DateTime<FixedOffset>>,
    /// The YouVersion Platform User id that was logged into the dev portal and agreed to the license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yvp_user_id: Option<String>,
}

impl V1LicensesCollectionGetResponseDataItem {
    pub fn builder() -> V1LicensesCollectionGetResponseDataItemBuilder {
        <V1LicensesCollectionGetResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1LicensesCollectionGetResponseDataItemBuilder {
    id: Option<i64>,
    name: Option<String>,
    version: Option<i64>,
    organization_id: Option<String>,
    html: Option<String>,
    bible_ids: Option<Vec<i64>>,
    uri: Option<String>,
    agreed_dt: Option<DateTime<FixedOffset>>,
    yvp_user_id: Option<String>,
}

impl V1LicensesCollectionGetResponseDataItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn bible_ids(mut self, value: Vec<i64>) -> Self {
        self.bible_ids = Some(value);
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    pub fn agreed_dt(mut self, value: DateTime<FixedOffset>) -> Self {
        self.agreed_dt = Some(value);
        self
    }

    pub fn yvp_user_id(mut self, value: impl Into<String>) -> Self {
        self.yvp_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1LicensesCollectionGetResponseDataItem`].
    pub fn build(self) -> Result<V1LicensesCollectionGetResponseDataItem, BuildError> {
        Ok(V1LicensesCollectionGetResponseDataItem {
            id: self.id,
            name: self.name,
            version: self.version,
            organization_id: self.organization_id,
            html: self.html,
            bible_ids: self.bible_ids,
            uri: self.uri,
            agreed_dt: self.agreed_dt,
            yvp_user_id: self.yvp_user_id,
        })
    }
}
