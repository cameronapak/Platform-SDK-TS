pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Permissions {
    /// Permissions the user has granted to the calling app.
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl Permissions {
    pub fn builder() -> PermissionsBuilder {
        <PermissionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PermissionsBuilder {
    permissions: Option<Vec<String>>,
}

impl PermissionsBuilder {
    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Permissions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](PermissionsBuilder::permissions)
    pub fn build(self) -> Result<Permissions, BuildError> {
        Ok(Permissions {
            permissions: self
                .permissions
                .ok_or_else(|| BuildError::missing_field("permissions"))?,
        })
    }
}
