pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Licenses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<LicensesDataItem>>,
}

impl Licenses {
    pub fn builder() -> LicensesBuilder {
        <LicensesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LicensesBuilder {
    data: Option<Vec<LicensesDataItem>>,
}

impl LicensesBuilder {
    pub fn data(mut self, value: Vec<LicensesDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Licenses`].
    pub fn build(self) -> Result<Licenses, BuildError> {
        Ok(Licenses { data: self.data })
    }
}
