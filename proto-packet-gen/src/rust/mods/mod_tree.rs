use std::collections::HashMap;

use proto_packet_tree::{ModName, ModNameRef, ModPathRef, Project, QualifiedNameRef, WithTypeName};

use crate::rust::{Error, Naming};

/// A module tree.
#[derive(Debug, Default)]
pub struct ModTree {
    pub(in crate::rust) subtrees: HashMap<ModName, ModTree>,
}

impl ModTree {
    //! Construction

    /// Creates a new mod tree from the `project`.
    pub fn new(naming: &Naming, project: &Project) -> Result<Self, Error> {
        let mut mod_tree: Self = Self::default();

        for (mod_path, schema_file) in project.schemas() {
            for type_name in schema_file.type_decs().iter().map(|d| d.type_name()) {
                mod_tree.insert(
                    mod_path.to_ref().to_qualified_name(type_name).to_ref(),
                    mod_path.to_ref(),
                    naming.mod_name_for_type_name(type_name)?,
                )?;
            }
        }

        Ok(mod_tree)
    }
}

impl ModTree {
    //! Properties

    /// Checks if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.subtrees.is_empty()
    }
}

impl ModTree {
    //! Insert

    /// Inserts the `mod_path` and `type_name` into the tree.
    ///
    /// Returns `Err(..)` if there was a name conflict.
    pub fn insert(
        &mut self,
        context: QualifiedNameRef,
        mod_path: ModPathRef,
        type_name: ModName,
    ) -> Result<(), Error> {
        self.insert_recursive(context, mod_path.mod_names(), type_name)
    }

    fn insert_recursive<'a>(
        &mut self,
        context: QualifiedNameRef<'a>,
        mut mod_path: impl Iterator<Item = ModNameRef<'a>>,
        type_name: ModName,
    ) -> Result<(), Error> {
        if let Some(mod_name) = mod_path.next().map(|n| n.to_owned()) {
            self.insert_recursive_mod_name(context, mod_name, mod_path, type_name)
        } else {
            self.insert_recursive_type_name(context, type_name.to_owned())
        }
    }

    fn insert_recursive_mod_name<'a>(
        &mut self,
        context: QualifiedNameRef<'a>,
        mod_name: ModName,
        rest_of_mod_path: impl Iterator<Item = ModNameRef<'a>>,
        type_name: ModName,
    ) -> Result<(), Error> {
        if self.subtrees.contains_key(&mod_name) {
            self.subtrees.get_mut(&mod_name).unwrap().insert_recursive(
                context,
                rest_of_mod_path,
                type_name,
            )
        } else {
            let mut tree: ModTree = ModTree::default();
            tree.insert_recursive(context, rest_of_mod_path, type_name)?;
            self.subtrees.insert(mod_name, tree);
            Ok(())
        }
    }

    fn insert_recursive_type_name(
        &mut self,
        context: QualifiedNameRef,
        type_name: ModName,
    ) -> Result<(), Error> {
        if let Some(subtree) = self.subtrees.get(&type_name) {
            if subtree.is_empty() {
                Err(Error::DuplicateTypeName {
                    name: context.to_owned(),
                })
            } else {
                Err(Error::TypeNameConflictsWithModName {
                    name: context.to_owned(),
                })
            }
        } else {
            self.subtrees.insert(type_name, ModTree::default());
            Ok(())
        }
    }
}
