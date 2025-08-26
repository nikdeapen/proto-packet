use std::collections::HashMap;

use proto_packet_tree::{ModName, ModNameRef, ModPathRef, Project, TypeNameRef, WithTypeName};

use crate::rust::Error::{DuplicateModNameFromTypeName, InvalidModNameFromTypeName};
use crate::rust::{Error, Naming};

/// A module tree.
#[derive(Debug, Default)]
pub struct ModTree {
    pub(in crate::rust) subs: HashMap<ModName, ModTree>,
}

impl ModTree {
    //! Construction

    /// Creates a new mod tree from the `project`.
    pub fn create(naming: &Naming, project: &Project) -> Result<Self, Error> {
        let mut result: Self = Self::default();

        for (mod_path, schema_file) in project.schema_files() {
            for type_name in schema_file.type_decs().iter().map(|d| d.type_name()) {
                result.insert(naming, mod_path.to_ref(), type_name)?;
            }
        }

        Ok(result)
    }
}

impl ModTree {
    //! Properties

    /// Checks if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.subs.is_empty()
    }
}

impl ModTree {
    //! Insert

    fn insert(
        &mut self,
        naming: &Naming,
        mod_path: ModPathRef,
        type_name: TypeNameRef,
    ) -> Result<(), Error> {
        self.insert_recursive(naming, mod_path.mod_names(), mod_path, type_name)
    }

    fn insert_recursive<'a>(
        &mut self,
        naming: &Naming,
        mut mod_names: impl Iterator<Item = ModNameRef<'a>>,
        mod_path: ModPathRef,
        type_name: TypeNameRef,
    ) -> Result<(), Error> {
        if let Some(mod_name) = mod_names.next() {
            if let Some(entry) = self.subs.get_mut(mod_name.as_ref()) {
                entry.insert_recursive(naming, mod_names, mod_path, type_name)
            } else {
                let mut tree: ModTree = ModTree::default();
                tree.insert_recursive(naming, mod_names, mod_path, type_name)?;
                self.subs.insert(mod_name.to_owned(), tree);
                Ok(())
            }
        } else {
            self.insert_leaf(naming, mod_path, type_name)
        }
    }

    fn insert_leaf(
        &mut self,
        naming: &Naming,
        mod_path: ModPathRef,
        type_name: TypeNameRef,
    ) -> Result<(), Error> {
        let mod_name: ModName =
            ModName::new(naming.mod_name_for_type_name(type_name)).map_err(|error_message| {
                InvalidModNameFromTypeName {
                    qualified_name: mod_path.to_qualified_name(type_name),
                    error_message,
                }
            })?;
        if self.subs.contains_key(&mod_name) {
            Err(DuplicateModNameFromTypeName {
                qualified_name: mod_path.to_qualified_name(type_name),
                mod_name,
            })
        } else {
            self.subs.insert(mod_name, ModTree::default());
            Ok(())
        }
    }
}
