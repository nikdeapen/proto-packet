use crate::{ModPath, SchemaFile};
use std::collections::HashMap;

/// A project.
#[derive(Clone, Debug, Default)]
pub struct Project {
    schema_files: HashMap<ModPath, SchemaFile>,
}

impl Project {
    //! Schema Files

    /// Gets the schema files.
    pub fn schema_files(&self) -> &HashMap<ModPath, SchemaFile> {
        &self.schema_files
    }

    /// Adds the `schema_file`.
    ///
    /// # Unsafe
    /// The `mod_path` must be able to be added.
    pub unsafe fn add_schema_file(&mut self, mod_path: ModPath, schema_file: SchemaFile) {
        debug_assert!(!self.schema_files.contains_key(&mod_path));

        self.schema_files.insert(mod_path, schema_file);
    }
}
