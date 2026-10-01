pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V1OrganizationsCollectionGetResponseDataItemAddressCountry {
    /// The short name of the place, e.g. "OK" for Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// The long name of the place, e.g. "Oklahoma" for the state of Oklahoma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
}

impl V1OrganizationsCollectionGetResponseDataItemAddressCountry {
    pub fn builder() -> V1OrganizationsCollectionGetResponseDataItemAddressCountryBuilder {
        <V1OrganizationsCollectionGetResponseDataItemAddressCountryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsCollectionGetResponseDataItemAddressCountryBuilder {
    short_name: Option<String>,
    long_name: Option<String>,
}

impl V1OrganizationsCollectionGetResponseDataItemAddressCountryBuilder {
    pub fn short_name(mut self, value: impl Into<String>) -> Self {
        self.short_name = Some(value.into());
        self
    }

    pub fn long_name(mut self, value: impl Into<String>) -> Self {
        self.long_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsCollectionGetResponseDataItemAddressCountry`].
    pub fn build(
        self,
    ) -> Result<V1OrganizationsCollectionGetResponseDataItemAddressCountry, BuildError> {
        Ok(V1OrganizationsCollectionGetResponseDataItemAddressCountry {
            short_name: self.short_name,
            long_name: self.long_name,
        })
    }
}
