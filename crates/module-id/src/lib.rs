//! Wrapper type for serializing Move module IDs as strings.

pub use account_address::{AccountAddress, AccountAddressData};
use anyhow::Result;
pub use move_core_types::{ident_str, identifier::Identifier};
use move_core_types::{language_storage::StructTag, parser::parse_struct_tag};
use schemars::{
    schema::{InstanceType, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, ops::Deref, str::FromStr};

// Re-export some types
pub use move_core_types::language_storage::ModuleId;

/// Wrapper around [ModuleId] which is serialized as a string.
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct ModuleIdData(ModuleId);

impl JsonSchema for ModuleIdData {
    fn schema_name() -> String {
        "ModuleId".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation {
                pattern: Some(format!(
                    "^0x[a-fA-F0-9]{{1,{}}}::[\\w]+$",
                    AccountAddress::LENGTH
                )),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl ModuleIdData {
    /// Creates a new [ModuleIdData].
    pub fn new(address: AccountAddress, ident: &str) -> Result<ModuleIdData> {
        Ok(ModuleIdData(ModuleId::new(
            address,
            Identifier::new(ident)?,
        )))
    }

    /// Gets the JSON-serializable address.
    pub fn address_data(&self) -> AccountAddressData {
        (*self.0.address()).into()
    }

    /// Gets the [ModuleId].
    pub fn inner(&self) -> &ModuleId {
        &self.0
    }
}

impl Deref for ModuleIdData {
    type Target = ModuleId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ModuleIdData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.short_str_lossless().fmt(f)
    }
}

impl Serialize for ModuleIdData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.short_str_lossless())
    }
}

impl From<ModuleId> for ModuleIdData {
    fn from(id: ModuleId) -> Self {
        Self(id)
    }
}

impl From<ModuleIdData> for ModuleId {
    fn from(id: ModuleIdData) -> Self {
        id.0
    }
}

impl From<ModuleIdData> for AccountAddress {
    fn from(val: ModuleIdData) -> Self {
        *val.0.address()
    }
}

impl<'de> Deserialize<'de> for ModuleIdData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ModuleIdData::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for ModuleIdData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tt: StructTag = parse_struct_tag(&format!("{}::Dummy", s))?;
        Ok(ModuleIdData(tt.module_id()))
    }
}

/// Parses a module ID.
///
/// # Example
///
/// ```
/// use module_id::*;
/// let id: ModuleIdData = parse_module_id("0x1::Errors").unwrap();
/// assert_eq!("0x1::Errors", id.to_string());
/// ```
pub fn parse_module_id(raw: &str) -> Result<ModuleIdData> {
    ModuleIdData::from_str(raw)
}

#[cfg(test)]
mod tests {
    use crate::{parse_module_id, ModuleIdData};

    #[test]
    fn test_serde() {
        let my_module_id = parse_module_id("0x1::A").unwrap();

        let ser = serde_json::to_string(&my_module_id).unwrap();
        let des: ModuleIdData = serde_json::from_str(&ser).unwrap();

        assert_eq!(my_module_id, des);
    }
}
