use crate::config::RustModConfig;

/// A Rust code generation config.
#[derive(Clone, Debug, Default)]
pub struct GenRustConfig {
    pub mods: RustModConfig,
}
