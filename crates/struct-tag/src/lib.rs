//! Wrapper type for serializing Move struct tags as strings.

use anyhow::Result;
pub use move_core_types::language_storage::StructTag;
use move_core_types::{account_address::AccountAddress, parser::parse_struct_tag};
use schemars::{
    schema::{InstanceType, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, ops::Deref, str::FromStr};

/// Wrapper around [StructTag] which is serialized as a string.
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct StructTagData(StructTag);

impl JsonSchema for StructTagData {
    fn schema_name() -> String {
        "StructTag".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation {
                pattern: Some(format!(
                    "^0x[a-fA-F0-9]{{1,{}}}::[\\w]+::[\\w]+$",
                    AccountAddress::LENGTH
                )),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl Deref for StructTagData {
    type Target = StructTag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for StructTagData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for StructTagData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl From<&StructTag> for StructTagData {
    fn from(id: &StructTag) -> Self {
        Self(id.clone())
    }
}

impl From<StructTag> for StructTagData {
    fn from(id: StructTag) -> Self {
        Self(id)
    }
}

impl From<&StructTagData> for StructTag {
    fn from(id: &StructTagData) -> Self {
        id.0.clone()
    }
}

impl From<StructTagData> for StructTag {
    fn from(id: StructTagData) -> Self {
        id.0
    }
}

impl<'de> Deserialize<'de> for StructTagData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(parse_struct_tag(&s)
            .map_err(serde::de::Error::custom)?
            .into())
    }
}

impl FromStr for StructTagData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_struct_tag(&format!("{}::Dummy", s))?.into())
    }
}

/// Parses a module ID.
///
/// # Example
///
/// ```
/// use move_core_types::language_storage::StructTag;
/// let id: StructTag = module_id::parse_module_id("0x1::Errors").unwrap().into();
/// assert_eq!("0x1::Errors", id.short_str_lossless());
/// ```
pub fn parse_module_id(raw: &str) -> Result<StructTagData> {
    StructTagData::from_str(raw)
}

#[cfg(test)]
mod tests {
    use crate::StructTagData;
    use move_core_types::parser::parse_struct_tag;

    #[test]
    fn test_serde() {
        let my_struct_tag: StructTagData = parse_struct_tag("0x1::A::B").unwrap().into();

        let ser = serde_json::to_string(&my_struct_tag).unwrap();
        let des: StructTagData = serde_json::from_str(&ser).unwrap();

        assert_eq!(my_struct_tag, des);
        assert_eq!(my_struct_tag.to_string(), "0x1::A::B");
    }
}
