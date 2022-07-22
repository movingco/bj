//! Wrapper type for serializing Move struct tags as strings.

use anyhow::Result;
pub use module_id::*;
pub use move_core_types::language_storage::StructTag;
use move_core_types::{
    account_address::AccountAddress, parser::parse_struct_tag as parse_struct_tag_move,
};
use schemars::{
    schema::{InstanceType, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, ops::Deref, str::FromStr};

/// Wrapper around [StructTag] which is serialized as a string.
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct StructTagData(StructTag);

impl StructTagData {
    /// Gets the serializable address.
    pub fn address_data(&self) -> AccountAddressData {
        self.into()
    }

    /// Gets the serializable module ID.
    pub fn module_id_data(&self) -> ModuleIdData {
        self.into()
    }

    /// Gets the [StructTag].
    pub fn inner(&self) -> &StructTag {
        self.into()
    }
}

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

impl From<StructTag> for StructTagData {
    fn from(id: StructTag) -> Self {
        Self(id)
    }
}

impl From<StructTagData> for StructTag {
    fn from(id: StructTagData) -> Self {
        id.0
    }
}

impl<'a> From<&'a StructTagData> for &'a StructTag {
    fn from(id: &'a StructTagData) -> Self {
        &id.0
    }
}

impl From<StructTagData> for AccountAddress {
    fn from(val: StructTagData) -> Self {
        val.0.address
    }
}

impl From<&StructTagData> for AccountAddressData {
    fn from(val: &StructTagData) -> Self {
        val.0.address.into()
    }
}

impl From<StructTagData> for ModuleId {
    fn from(val: StructTagData) -> Self {
        val.0.module_id()
    }
}

impl From<&StructTagData> for ModuleIdData {
    fn from(val: &StructTagData) -> Self {
        val.0.module_id().into()
    }
}

impl<'de> Deserialize<'de> for StructTagData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_struct_tag(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for StructTagData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_struct_tag_move(s)?.into())
    }
}

/// Parses a struct tag.
///
/// # Example
///
/// ```
/// use struct_tag::*;
/// let id: StructTagData = struct_tag::parse_struct_tag("0x1::Errors::Errors").unwrap();
/// assert_eq!("0x1::Errors::Errors", id.to_string());
/// ```
pub fn parse_struct_tag(raw: &str) -> Result<StructTagData> {
    StructTagData::from_str(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let my_struct_tag: StructTagData = parse_struct_tag("0x1::A::B").unwrap();

        let ser = serde_json::to_string(&my_struct_tag).unwrap();
        let des: StructTagData = serde_json::from_str(&ser).unwrap();

        assert_eq!(my_struct_tag, des);
        assert_eq!(my_struct_tag.to_string(), "0x1::A::B");
    }
}
