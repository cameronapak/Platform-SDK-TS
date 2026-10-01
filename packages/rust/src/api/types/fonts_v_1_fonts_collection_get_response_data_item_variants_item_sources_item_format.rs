pub use crate::prelude::*;

/// The file format for this source asset.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat {
    Woff2,
    Ttf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Woff2 => serializer.serialize_str("woff2"),
            Self::Ttf => serializer.serialize_str("ttf"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "woff2" => Ok(Self::Woff2),
            "ttf" => Ok(Self::Ttf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for V1FontsCollectionGetResponseDataItemVariantsItemSourcesItemFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Woff2 => write!(f, "woff2"),
            Self::Ttf => write!(f, "ttf"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
