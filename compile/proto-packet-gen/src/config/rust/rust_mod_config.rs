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

impl RustModConfig {
    //! Properties

    /// Gets the mod file path.
    #[cfg(feature = "rust")]
    pub fn file_path(
        &self,
        target: &file_storage::FolderPath,
        root: bool,
    ) -> Result<file_storage::FilePath, crate::rust::Error> {
        let mod_file_name: &str = if root && self.use_lib_root {
            self.lib_file_name.as_str()
        } else {
            self.mod_file_name.as_str()
        };

        target
            .clone_with_extra_capacity(mod_file_name.len())
            .to_path()
            .with_appended(mod_file_name)
            .to_file()
            .map_err(|file_path| crate::rust::Error::InvalidModFileName {
                file_path: file_path.export_path(),
                mod_file_name: mod_file_name.to_string(),
            })
    }
}
