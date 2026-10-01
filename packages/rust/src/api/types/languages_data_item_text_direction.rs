pub use crate::prelude::*;

/// Default text direction for this language. ltr is left to right and rtl is right to left.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguagesDataItemTextDirection {
    Ltr,
    Rtl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LanguagesDataItemTextDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ltr => serializer.serialize_str("ltr"),
            Self::Rtl => serializer.serialize_str("rtl"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LanguagesDataItemTextDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ltr" => Ok(Self::Ltr),
            "rtl" => Ok(Self::Rtl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LanguagesDataItemTextDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ltr => write!(f, "ltr"),
            Self::Rtl => write!(f, "rtl"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
