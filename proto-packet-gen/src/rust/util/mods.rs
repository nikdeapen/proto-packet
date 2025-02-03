use file_storage::{FilePath, FolderPath};

use crate::rust::Error;

/// A mod file config.
#[derive(Clone, Debug)]
pub struct Mods {
    use_lib_root: bool,
    lib_file_name: String,
    mod_file_name: String,
}

impl Default for Mods {
    fn default() -> Self {
        Self {
            use_lib_root: true,
            lib_file_name: "lib.rs".to_string(),
            mod_file_name: "mod.rs".to_string(),
        }
    }
}

impl Mods {
    //! Properties

    /// Gets the mod file path.
    pub fn file_path(&self, target: &FolderPath, root: bool) -> Result<FilePath, Error> {
        let file_name: &str = if root && self.use_lib_root {
            self.lib_file_name.as_str()
        } else {
            self.mod_file_name.as_str()
        };
        target
            .clone_with_extra_capacity(file_name.len())
            .to_path()
            .with_appended(file_name)
            .to_file()
            .ok_or(Error::InvalidModFileName {
                file_name: file_name.to_string(),
            })
    }
}
