/// A rust mod file config.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct RustModConfig {
    pub retain: Vec<String>,
    pub use_lib_root: bool,
    pub lib_file_name: String,
    pub mod_file_name: String,
}

impl Default for RustModConfig {
    fn default() -> Self {
        Self {
            retain: vec![],
            use_lib_root: default_use_lib_root(),
            lib_file_name: default_lib_file_name(),
            mod_file_name: default_mod_file_name(),
        }
    }
}

fn default_use_lib_root() -> bool {
    true
}

fn default_lib_file_name() -> String {
    "lib.rs".to_string()
}

fn default_mod_file_name() -> String {
    "mod.rs".to_string()
}
