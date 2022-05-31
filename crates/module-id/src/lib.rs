//! Wrapper type for serializing Move module IDs as strings.

use move_core_types::{
    language_storage::{ModuleId, StructTag},
    parser::parse_struct_tag,
};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;

/// Wrapper around [ModuleId] which can be serialized to JSON.
#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct ModuleIdData(ModuleId);

impl Display for ModuleIdData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
        let tt: StructTag =
            parse_struct_tag(&format!("{}::Dummy", &s)).map_err(serde::de::Error::custom)?;
        Ok(ModuleIdData(tt.module_id()))
    }
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

        println!("{}", &des);
    }
}
