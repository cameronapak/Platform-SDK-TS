pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationsDataItemAddressLocality {
    /// The short name of the place, e.g. "OK" for Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// The long name of the place, e.g. "Oklahoma" for the state of Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
}

impl OrganizationsDataItemAddressLocality {
    pub fn builder() -> OrganizationsDataItemAddressLocalityBuilder {
        <OrganizationsDataItemAddressLocalityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationsDataItemAddressLocalityBuilder {
    short_name: Option<String>,
    long_name: Option<String>,
}

impl OrganizationsDataItemAddressLocalityBuilder {
    pub fn short_name(mut self, value: impl Into<String>) -> Self {
        self.short_name = Some(value.into());
        self
    }

    pub fn long_name(mut self, value: impl Into<String>) -> Self {
        self.long_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrganizationsDataItemAddressLocality`].
    pub fn build(self) -> Result<OrganizationsDataItemAddressLocality, BuildError> {
        Ok(OrganizationsDataItemAddressLocality {
            short_name: self.short_name,
            long_name: self.long_name,
        })
    }
}
