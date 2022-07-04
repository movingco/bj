//! Wrapper type for serializing Move module IDs as strings.

use anyhow::Result;
use move_core_types::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    parser::parse_struct_tag,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, str::FromStr};

/// Wrapper around [ModuleId] which is serialized as a string.
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct ModuleIdData(ModuleId);

impl ModuleIdData {
    /// Gets the address of the module.
    pub fn address(&self) -> &AccountAddress {
        self.0.address()
    }

    /// Gets the name of the module as a [String].
    pub fn name(&self) -> &str {
        self.0.name().as_str()
    }

    /// Renders the [ModuleIdData] as a short string.
    ///
    /// # Example
    ///
    /// ```
    /// use module_id::*;
    /// let id: ModuleIdData = module_id::parse_module_id("0x1::Errors").unwrap();
    /// assert_eq!("0x1::Errors", id.short_str_lossless());
    /// ```
    pub fn short_str_lossless(&self) -> String {
        self.0.short_str_lossless()
    }

    /// Gets the [ModuleId].
    pub fn inner(&self) -> &ModuleId {
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

impl From<&ModuleId> for ModuleIdData {
    fn from(id: &ModuleId) -> Self {
        Self(id.clone())
    }
}

impl From<ModuleId> for ModuleIdData {
    fn from(id: ModuleId) -> Self {
        Self(id)
    }
}

impl From<&ModuleIdData> for ModuleId {
    fn from(id: &ModuleIdData) -> Self {
        id.0.clone()
    }
}

impl From<ModuleIdData> for ModuleId {
    fn from(id: ModuleIdData) -> Self {
        id.0
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
/// use move_core_types::language_storage::ModuleId;
/// let id: ModuleId = module_id::parse_module_id("0x1::Errors").unwrap().into();
/// assert_eq!("0x1::Errors", id.short_str_lossless());
/// ```
pub fn parse_module_id(raw: &str) -> Result<ModuleIdData> {
    ModuleIdData::from_str(raw)
}

#[cfg(test)]
mod tests {
    use move_core_types::parser::parse_struct_tag;

    use crate::ModuleIdData;

    #[test]
    fn test_serde() {
        let my_module_id = ModuleIdData(parse_struct_tag("0x1::A::B").unwrap().module_id());

        let ser = serde_json::to_string(&my_module_id).unwrap();
        let des: ModuleIdData = serde_json::from_str(&ser).unwrap();

        assert_eq!(my_module_id, des);
    }
}
