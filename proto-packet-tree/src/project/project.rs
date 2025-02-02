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

    /// Adds the schema.
    ///
    /// Returns `false` if the `mod_path` is already present.
    pub fn add_schema<P>(&mut self, mod_path: P, schema_file: SchemaFile) -> bool
    where
        P: Into<ModPath>,
    {
        let mod_path: ModPath = mod_path.into();
        if self.schemas.contains_key(&mod_path) {
            false
        } else {
            self.schemas.insert(mod_path.into(), schema_file);
            true
        }
    }
}
