use crate::config::RustModConfig;
use serde::{Deserialize, Serialize};

/// A rust code generation config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RustConfig {
    #[serde(default)]
    pub mods: RustModConfig,
}
