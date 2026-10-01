pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppSummariesDataItemStatus {
    Development,
    Live,
    Archived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppSummariesDataItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Development => serializer.serialize_str("development"),
            Self::Live => serializer.serialize_str("live"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppSummariesDataItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "development" => Ok(Self::Development),
            "live" => Ok(Self::Live),
            "archived" => Ok(Self::Archived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppSummariesDataItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Live => write!(f, "live"),
            Self::Archived => write!(f, "archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
