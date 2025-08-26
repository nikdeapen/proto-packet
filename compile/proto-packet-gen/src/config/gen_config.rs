use crate::config::RustConfig;

/// A rust code generation config.
#[derive(Clone, Debug, Default)]
pub struct GenConfig {
    pub rust: RustConfig,
}
