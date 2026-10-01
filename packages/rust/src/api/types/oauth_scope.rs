pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OauthScope {
    /// Read highlights access
    ReadHighlights,
    /// Write highlights access
    WriteHighlights,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OauthScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReadHighlights => serializer.serialize_str("read_highlights"),
            Self::WriteHighlights => serializer.serialize_str("write_highlights"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OauthScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "read_highlights" => Ok(Self::ReadHighlights),
            "write_highlights" => Ok(Self::WriteHighlights),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OauthScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadHighlights => write!(f, "read_highlights"),
            Self::WriteHighlights => write!(f, "write_highlights"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
