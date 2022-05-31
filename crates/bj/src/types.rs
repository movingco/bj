//! Serializable representation of common types.

use module_id::ModuleIdData;
use move_core_types::errmap::ErrorDescription;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMapping {
    /// The set of error categories and their descriptions
    pub error_categories: BTreeMap<u64, ErrorDescription>,
    /// The set of modules, and the module-specific errors
    pub module_error_maps: BTreeMap<ModuleIdData, BTreeMap<u64, ErrorDescription>>,
}

impl From<move_core_types::errmap::ErrorMapping> for ErrorMapping {
    fn from(errmap: move_core_types::errmap::ErrorMapping) -> Self {
        ErrorMapping {
            error_categories: errmap.error_categories.into(),
            module_error_maps: errmap
                .module_error_maps
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl Into<move_core_types::errmap::ErrorMapping> for ErrorMapping {
    fn into(self) -> move_core_types::errmap::ErrorMapping {
        move_core_types::errmap::ErrorMapping {
            error_categories: self.error_categories.into(),
            module_error_maps: self
                .module_error_maps
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}
