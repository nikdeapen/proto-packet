use crate::rust::util::Naming;
use crate::rust::Error;
use crate::rust::Error::DuplicateModName;
use proto_packet_tree::{ModName, ModNameRef, ModPathRef, TypeNameRef};
use std::collections::HashMap;

/// A module tree.
#[derive(Debug, Default)]
pub struct ModTree {
    pub(in crate::rust) subs: HashMap<ModName, ModTree>,
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

    /// Adds the `type_name` within the `mod_path`.
    pub fn insert(
        &mut self,
        naming: &Naming,
        mod_path: ModPathRef,
        type_name: TypeNameRef,
    ) -> Result<(), Error> {
        self.insert_recursive(naming, mod_path.mod_names(), mod_path, type_name)
    }

    /// Recursively adds the `type_name` within the `mod_path`.
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

    /// Adds the `type_name` within the `mod_path` as a leaf node.
    fn insert_leaf(
        &mut self,
        naming: &Naming,
        mod_path: ModPathRef,
        type_name: TypeNameRef,
    ) -> Result<(), Error> {
        let mod_name: ModName = naming.mod_name_for_type_name(type_name)?;
        if self.subs.contains_key(&mod_name) {
            Err(DuplicateModName {
                qualified_name: mod_path.to_qualified_name(type_name),
            })
        } else {
            self.subs.insert(mod_name, ModTree::default());
            Ok(())
        }
    }
}
