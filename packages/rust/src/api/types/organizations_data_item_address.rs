pub use crate::prelude::*;

/// The Address Schema belonging to the Organization Resource in the Platform.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationsDataItemAddress {
    /// The human-readable address of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_address: Option<String>,
    /// A less specific and more broad field that can be a combination of different regional fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_locality: Option<String>,
    /// The textual identifier that uniquely identifies a place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    /// The location of the address profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    /// The location of the address profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    #[serde(rename = "administrative_area_level_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_area_level1: Option<OrganizationsDataItemAddressAdministrativeAreaLevel1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<OrganizationsDataItemAddressLocality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<OrganizationsDataItemAddressCountry>,
}

impl OrganizationsDataItemAddress {
    pub fn builder() -> OrganizationsDataItemAddressBuilder {
        <OrganizationsDataItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationsDataItemAddressBuilder {
    formatted_address: Option<String>,
    formatted_locality: Option<String>,
    place_id: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    administrative_area_level1: Option<OrganizationsDataItemAddressAdministrativeAreaLevel1>,
    locality: Option<OrganizationsDataItemAddressLocality>,
    country: Option<OrganizationsDataItemAddressCountry>,
}

impl OrganizationsDataItemAddressBuilder {
    pub fn formatted_address(mut self, value: impl Into<String>) -> Self {
        self.formatted_address = Some(value.into());
        self
    }

    pub fn formatted_locality(mut self, value: impl Into<String>) -> Self {
        self.formatted_locality = Some(value.into());
        self
    }

    pub fn place_id(mut self, value: impl Into<String>) -> Self {
        self.place_id = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn administrative_area_level1(
        mut self,
        value: OrganizationsDataItemAddressAdministrativeAreaLevel1,
    ) -> Self {
        self.administrative_area_level1 = Some(value);
        self
    }

    pub fn locality(mut self, value: OrganizationsDataItemAddressLocality) -> Self {
        self.locality = Some(value);
        self
    }

    pub fn country(mut self, value: OrganizationsDataItemAddressCountry) -> Self {
        self.country = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationsDataItemAddress`].
    pub fn build(self) -> Result<OrganizationsDataItemAddress, BuildError> {
        Ok(OrganizationsDataItemAddress {
            formatted_address: self.formatted_address,
            formatted_locality: self.formatted_locality,
            place_id: self.place_id,
            latitude: self.latitude,
            longitude: self.longitude,
            administrative_area_level1: self.administrative_area_level1,
            locality: self.locality,
            country: self.country,
        })
    }
}
