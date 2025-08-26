use file_storage::{FilePath, FolderPath};
use std::collections::HashMap;

/// Represents generated source code.
#[derive(Debug)]
pub struct GeneratedCode {
    target: FolderPath,
    sources: HashMap<FilePath, String>,
}

impl From<FolderPath> for GeneratedCode {
    fn from(target: FolderPath) -> Self {
        Self {
            target,
            sources: HashMap::default(),
        }
    }
}

impl GeneratedCode {
    //! Target Folder

    /// Gets the target folder.
    pub fn target(&self) -> &FolderPath {
        &self.target
    }
}

impl GeneratedCode {
    //! Sources

    /// Gets the sources.
    pub fn sources(&self) -> &HashMap<FilePath, String> {
        &self.sources
    }

    /// Checks if the `file_path` can be added.
    pub fn can_add_file_path(&self, file_path: &FilePath) -> bool {
        file_path.as_str().starts_with(self.target.as_str())
            && !self.sources.contains_key(file_path)
    }

    /// Adds the source.
    ///
    /// # Unsafe
    /// The `file_path` must be able to be added.
    pub unsafe fn add_source(&mut self, file_path: FilePath, source: String) {
        debug_assert!(self.can_add_file_path(&file_path));

        self.sources.insert(file_path, source);
    }

    /// Adds the source.
    ///
    /// # Unsafe
    /// The `file_path` must be able to be added.
    pub unsafe fn with_source(mut self, file_path: FilePath, source_file: String) -> Self {
        self.add_source(file_path, source_file);
        self
    }
}
