use std::collections::HashMap;

use file_storage::{FilePath, FolderPath};

/// Represents generated source code.
#[derive(Debug)]
pub struct Generated {
    target: FolderPath,
    sources: HashMap<FilePath, String>,
}

impl From<FolderPath> for Generated {
    fn from(target: FolderPath) -> Self {
        Self {
            target,
            sources: HashMap::default(),
        }
    }
}

impl Generated {
    //! Target Folder

    /// Gets the target folder.
    pub fn target(&self) -> &FolderPath {
        &self.target
    }
}

impl Generated {
    //! Sources

    /// Gets the sources.
    pub fn sources(&self) -> &HashMap<FilePath, String> {
        &self.sources
    }

    /// Adds the source.
    pub fn add_source(&mut self, file_path: FilePath, source_file: String) -> bool {
        if self.sources.contains_key(&file_path) {
            false
        } else {
            self.sources.insert(file_path, source_file);
            true
        }
    }

    /// Adds the source.
    pub fn with_source(mut self, file_path: FilePath, source_file: String) -> Self {
        self.add_source(file_path, source_file);
        self
    }
}
