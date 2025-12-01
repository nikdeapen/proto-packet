use crate::config::GenRustConfig;

/// A code generation config.
#[derive(Clone, Debug, Default)]
pub struct GenConfig {
    pub rust: GenRustConfig,
}
