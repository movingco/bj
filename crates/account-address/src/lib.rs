//! Wrapper type for serializing Move account addresses as strings.

use anyhow::Result;
pub use move_core_types::account_address::AccountAddress;
use schemars::{
    schema::{InstanceType, SchemaObject, StringValidation},
    JsonSchema,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, ops::Deref, str::FromStr};

/// Wrapper around [AccountAddress] which is serialized as a string.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountAddressData(AccountAddress);

impl JsonSchema for AccountAddressData {
    fn schema_name() -> String {
        "AccountAddress".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation {
                pattern: Some(format!("^0x[a-fA-F0-9]{{1,{}}}$", AccountAddress::LENGTH)),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl Deref for AccountAddressData {
    type Target = AccountAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for AccountAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_hex_literal().fmt(f)
    }
}

impl Serialize for AccountAddressData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_hex_literal())
    }
}

impl From<AccountAddress> for AccountAddressData {
    fn from(id: AccountAddress) -> Self {
        Self(id)
    }
}

impl From<AccountAddressData> for AccountAddress {
    fn from(id: AccountAddressData) -> Self {
        id.0
    }
}

impl<'de> Deserialize<'de> for AccountAddressData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AccountAddressData::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for AccountAddressData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountAddress::from_hex_literal(s)
            .map_err(|e| anyhow::anyhow!("error parsing account address `{}`: {}", s, e))?
            .into())
    }
}

#[cfg(test)]
mod tests {
    use move_core_types::account_address::AccountAddress;

    use crate::AccountAddressData;

    #[test]
    fn test_serde() {
        let my_module_id = AccountAddressData(AccountAddress::from_hex_literal("0x123").unwrap());

        let ser = serde_json::to_string(&my_module_id).unwrap();
        let des: AccountAddressData = serde_json::from_str(&ser).unwrap();

        assert_eq!(my_module_id, des);
    }
}
