//! Move error map which can be represented as JSON.

use module_id::ModuleIdData;
use move_core_types::errmap::ErrorDescription;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Move error map which can be represented as JSON.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ErrorMapping {
    /// The set of error categories and their descriptions
    pub error_categories: BTreeMap<u64, ErrorDescription>,
    /// The set of modules, and the module-specific errors
    pub module_error_maps: BTreeMap<ModuleIdData, BTreeMap<u64, ErrorDescription>>,
}

impl Extend<ErrorMapping> for ErrorMapping {
    fn extend<T: IntoIterator<Item = ErrorMapping>>(&mut self, iter: T) {
        iter.into_iter().fold(self, |acc, map| {
            acc.error_categories.extend(map.error_categories);
            acc.module_error_maps.extend(map.module_error_maps);
            acc
        });
    }
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
