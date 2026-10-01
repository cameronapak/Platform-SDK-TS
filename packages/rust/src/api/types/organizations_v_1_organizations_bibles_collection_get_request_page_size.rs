pub use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum V1OrganizationsBiblesCollectionGetRequestPageSize {
    All,
    Numeric(u8),
}

impl Serialize for V1OrganizationsBiblesCollectionGetRequestPageSize {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("*"),
            Self::Numeric(value) => serializer.serialize_u8(*value),
        }
    }
}
impl<'de> Deserialize<'de> for V1OrganizationsBiblesCollectionGetRequestPageSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(value) if value == "*" => Ok(Self::All),
            serde_json::Value::Number(value) => value
                .as_u64()
                .filter(|value| (1..=99).contains(value))
                .map(|value| Self::Numeric(value as u8))
                .ok_or_else(|| serde::de::Error::custom("page size must be 1..=99 or *")),
            _ => Err(serde::de::Error::custom("page size must be 1..=99 or *")),
        }
    }
}
impl std::str::FromStr for V1OrganizationsBiblesCollectionGetRequestPageSize {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "*" {
            return Ok(Self::All);
        }
        value
            .parse::<u8>()
            .ok()
            .filter(|value| (1..=99).contains(value))
            .map(Self::Numeric)
            .ok_or("page size must be 1..=99 or *")
    }
}
impl fmt::Display for V1OrganizationsBiblesCollectionGetRequestPageSize {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => formatter.write_str("*"),
            Self::Numeric(value) => write!(formatter, "{value}"),
        }
    }
}
