pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum V1SearchUnifiedCollectionGetRequestUserIntent {
    Unknown,
    Topical,
    Text,
    Reference,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for V1SearchUnifiedCollectionGetRequestUserIntent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::Topical => serializer.serialize_str("topical"),
            Self::Text => serializer.serialize_str("text"),
            Self::Reference => serializer.serialize_str("reference"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for V1SearchUnifiedCollectionGetRequestUserIntent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unknown" => Ok(Self::Unknown),
            "topical" => Ok(Self::Topical),
            "text" => Ok(Self::Text),
            "reference" => Ok(Self::Reference),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for V1SearchUnifiedCollectionGetRequestUserIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Topical => write!(f, "topical"),
            Self::Text => write!(f, "text"),
            Self::Reference => write!(f, "reference"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
