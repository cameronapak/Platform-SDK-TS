pub use crate::prelude::*;

/// Canon identifier
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BiblesIndexCollectionGetResponseBooksItemCanon {
    NewTestament,
    OldTestament,
    Deuterocanon,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BiblesIndexCollectionGetResponseBooksItemCanon {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NewTestament => serializer.serialize_str("new_testament"),
            Self::OldTestament => serializer.serialize_str("old_testament"),
            Self::Deuterocanon => serializer.serialize_str("deuterocanon"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BiblesIndexCollectionGetResponseBooksItemCanon {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new_testament" => Ok(Self::NewTestament),
            "old_testament" => Ok(Self::OldTestament),
            "deuterocanon" => Ok(Self::Deuterocanon),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BiblesIndexCollectionGetResponseBooksItemCanon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewTestament => write!(f, "new_testament"),
            Self::OldTestament => write!(f, "old_testament"),
            Self::Deuterocanon => write!(f, "deuterocanon"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
