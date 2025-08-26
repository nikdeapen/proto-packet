use crate::config::RustConfig;
use serde::{Deserialize, Serialize};

/// A rust code generation config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenConfig {
    #[serde(default)]
    pub rust: RustConfig,
}
