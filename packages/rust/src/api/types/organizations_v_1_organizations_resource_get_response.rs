pub use crate::prelude::*;

/// The Organization Resource in the Platform.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct V1OrganizationsResourceGetResponse {
    /// The unique identifier of the organization in the Platform.
    #[serde(default)]
    pub id: String,
    /// The id of the parent organization if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_organization_id: Option<String>,
    /// Publisher's name in the language negotiated by Accept-Language headers. If none match known translations, then the primary language of the publisher is used. Whichever language is chosen will be sent back in the Content-Language header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the organization. It's purpose and goals, values and mission, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The contact email address for the organization if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The contact phone number for the organization if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The primary language of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    /// The web site for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    /// The Address Schema belonging to the Organization Resource in the Platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<V1OrganizationsResourceGetResponseAddress>,
}

impl V1OrganizationsResourceGetResponse {
    pub fn builder() -> V1OrganizationsResourceGetResponseBuilder {
        <V1OrganizationsResourceGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V1OrganizationsResourceGetResponseBuilder {
    id: Option<String>,
    parent_organization_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    primary_language: Option<String>,
    website_url: Option<String>,
    address: Option<V1OrganizationsResourceGetResponseAddress>,
}

impl V1OrganizationsResourceGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn parent_organization_id(mut self, value: impl Into<String>) -> Self {
        self.parent_organization_id = Some(value.into());
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

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(value.into());
        self
    }

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    pub fn address(mut self, value: V1OrganizationsResourceGetResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`V1OrganizationsResourceGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](V1OrganizationsResourceGetResponseBuilder::id)
    pub fn build(self) -> Result<V1OrganizationsResourceGetResponse, BuildError> {
        Ok(V1OrganizationsResourceGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            parent_organization_id: self.parent_organization_id,
            name: self.name,
            description: self.description,
            email: self.email,
            phone: self.phone,
            primary_language: self.primary_language,
            website_url: self.website_url,
            address: self.address,
        })
    }
}
