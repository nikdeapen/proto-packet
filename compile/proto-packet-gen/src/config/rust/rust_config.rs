use crate::config::RustModConfig;

/// A rust code generation config.
#[derive(Clone, Debug, Default)]
pub struct RustConfig {
    pub mods: RustModConfig,
}
