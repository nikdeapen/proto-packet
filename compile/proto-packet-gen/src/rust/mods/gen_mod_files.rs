use code_gen::{CodeBuffer, EmptyLine, Source, Statement, WithStatements};
use file_storage::{FilePath, FolderPath};

use crate::rust::GenRust;
use crate::rust::{Error, ModTree};
use crate::GeneratedCode;
use proto_packet_tree::{ModNameRef, Project};

impl GenRust {
    //! Gen Mod Files

    /// Generates the mod files for the `project` and adds them to `generated`.
    pub fn gen_mod_files(
        &self,
        project: &Project,
        generated: &mut GeneratedCode,
    ) -> Result<(), Error> {
        let mod_tree: ModTree = ModTree::create(&self.naming, project)?;
        let target: &FolderPath = &generated.target().clone();
        self.gen_mod_files_from_tree(&mod_tree, generated, target, true)
    }

    fn gen_mod_files_from_tree(
        &self,
        tree: &ModTree,
        generated: &mut GeneratedCode,
        target: &FolderPath,
        root: bool,
    ) -> Result<(), Error> {
        if !tree.is_empty() {
            let file_path: FilePath = self.config.rust.mods.file_path(target, root)?;
            let source: Source = self.mod_file_source(tree)?;
            let mut b: CodeBuffer = CodeBuffer::default();
            source.write(&mut b, 0);
            unsafe { generated.add_source(file_path, b.into()) };
        };
        for (mod_name, sub_tree) in &tree.subs {
            let target: FolderPath = target
                .clone_with_extra_capacity(
                    mod_name.len() + target.path().file_separator().len_utf8(),
                )
                .to_path()
                .with_appended(mod_name)
                .make_folder();
            self.gen_mod_files_from_tree(sub_tree, generated, &target, false)?;
        }
        Ok(())
    }

    fn mod_file_source(&self, tree: &ModTree) -> Result<Source, Error> {
        debug_assert!(!tree.is_empty());

        let mut mods: Vec<ModNameRef> = Vec::default();
        let mut pub_mods: Vec<ModNameRef> = Vec::default();

        for (mod_name, subtree) in &tree.subs {
            if subtree.is_empty() {
                mods.push(mod_name.to_ref());
            } else {
                pub_mods.push(mod_name.to_ref());
            }
        }

        let mut source: Source = Source::default();

        mods.sort();
        if !mods.is_empty() {
            for mod_name in &mods {
                if self.naming.is_keyword(mod_name) {
                    source.add_semi(format!("pub use r#{}::*", mod_name));
                } else {
                    source.add_semi(format!("pub use {}::*", mod_name));
                }
            }
            source.add_statement(EmptyLine::default());
            for mod_name in &mods {
                if self.naming.is_keyword(mod_name) {
                    source.add_semi(format!("mod r#{}", mod_name));
                } else {
                    source.add_semi(format!("mod {}", mod_name));
                }
            }
        }

        pub_mods.sort();
        if !pub_mods.is_empty() {
            if !mods.is_empty() {
                source.add_statement(EmptyLine::default());
            }
            for mod_name in &pub_mods {
                source.add_semi(format!("pub mod {}", mod_name));
            }
        }

        Ok(source)
    }
}
