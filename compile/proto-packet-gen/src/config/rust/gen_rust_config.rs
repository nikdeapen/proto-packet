use crate::config::RustModConfig;
use serde::{Deserialize, Serialize};

/// A Rust code generation config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenRustConfig {
    pub mods: RustModConfig,
}
