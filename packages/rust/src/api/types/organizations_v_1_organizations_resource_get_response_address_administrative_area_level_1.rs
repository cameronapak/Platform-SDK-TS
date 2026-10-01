pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1 {
    /// The short name of the place, e.g. "OK" for Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// The long name of the place, e.g. "Oklahoma" for the state of Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
}

impl V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1 {
    pub fn builder() -> V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1Builder {
        <V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1Builder {
    short_name: Option<String>,
    long_name: Option<String>,
}

impl V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1Builder {
    pub fn short_name(mut self, value: impl Into<String>) -> Self {
        self.short_name = Some(value.into());
        self
    }

    pub fn long_name(mut self, value: impl Into<String>) -> Self {
        self.long_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1`].
    pub fn build(
        self,
    ) -> Result<V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1, BuildError> {
        Ok(
            V1OrganizationsResourceGetResponseAddressAdministrativeAreaLevel1 {
                short_name: self.short_name,
                long_name: self.long_name,
            },
        )
    }
}
