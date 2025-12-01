use crate::config::GenRustConfig;
use serde::{Deserialize, Serialize};

/// A code generation config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenConfig {
    pub rust: GenRustConfig,
}
