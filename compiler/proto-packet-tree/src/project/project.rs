use std::collections::HashMap;

use crate::{ModPath, SchemaFile};

/// A project.
#[derive(Clone, Debug, Default)]
pub struct Project {
    schemas: HashMap<ModPath, SchemaFile>,
}

impl Project {
    //! Schemas

    /// Gets the schemas.
    pub fn schemas(&self) -> &HashMap<ModPath, SchemaFile> {
        &self.schemas
    }

    /// Gets the optional schema file with the given `mod_path`.
    /// todo -- &ModPath should be ModPathRef
    pub fn schema_with_mod_path(&self, mod_path: &ModPath) -> Option<&SchemaFile> {
        self.schemas.get(mod_path)
    }

    /// Adds the `schema_file` with the `mod_path`.
    ///
    /// # Unsafe
    /// The `mod_path` must not be present.
    pub unsafe fn add_schema<P>(&mut self, mod_path: P, schema_file: SchemaFile)
    where
        P: Into<ModPath>,
    {
        let mod_path: ModPath = mod_path.into();

        debug_assert!(!self.schemas.contains_key(&mod_path));

        self.schemas.insert(mod_path, schema_file);
    }
}
