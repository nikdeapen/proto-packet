use crate::rust::{GenRust, ModTree, Naming};
use crate::Writer;
use clerr::Report;
use code_gen::{EmptyLine, Source, WithStatements};
use proto_packet_tree::ModNameRef;

impl GenRust {
    //! Gen Files: Mods

    /// Generates the mod files for the `tree`.
    pub(in crate::rust) fn gen_mod_files<W>(&self, tree: &ModTree, writer: &W) -> Result<(), Report>
    where
        W: Writer,
    {
        self.gen_mod_files_recursive(tree, &mut String::from("/"), true, writer)
    }

    /// Recursively generates the mod files for the `tree`.
    fn gen_mod_files_recursive<W>(
        &self,
        tree: &ModTree,
        target: &mut String,
        is_root: bool,
        writer: &W,
    ) -> Result<(), Report>
    where
        W: Writer,
    {
        if !tree.is_empty() {
            let file_name: &str = self.config.rust.mods.file_name(is_root);
            target.push_str(file_name);
            let source: Source = self.gen_mod_file_source(tree)?;
            writer.write(&source, target.as_str())?;
            target.truncate(target.len() - file_name.len());
        };

        for (mod_name, sub_tree) in &tree.subs {
            target.push_str(mod_name.as_ref());
            target.push_str("/");
            self.gen_mod_files_recursive(sub_tree, target, false, writer)?;
            target.truncate(target.len() - mod_name.len() - 1);
        }

        Ok(())
    }

    /// Generates the source code for the mod file given the non-empty `tree`.
    fn gen_mod_file_source(&self, tree: &ModTree) -> Result<Source, Report> {
        debug_assert!(!tree.is_empty());

        fn source_mod_name(_naming: &Naming, mod_name: ModNameRef) -> String {
            mod_name.to_string()
        }

        let mut mods: Vec<ModNameRef> = Vec::default();
        let mut pub_mods: Vec<ModNameRef> = Vec::default();
        for (mod_name, subtree) in &tree.subs {
            if subtree.is_empty() {
                mods.push(mod_name.to_ref());
            } else {
                pub_mods.push(mod_name.to_ref());
            }
        }
        mods.sort();
        pub_mods.sort();

        let mut source: Source = Source::default();
        if !mods.is_empty() {
            for mod_name in &mods {
                source.add_semi(format!(
                    "pub use {}::*",
                    source_mod_name(&self.naming, *mod_name)
                ));
            }
            source.add_statement(EmptyLine::default());
            for mod_name in &mods {
                source.add_semi(format!("mod {}", source_mod_name(&self.naming, *mod_name)));
            }
        }

        if !pub_mods.is_empty() {
            if !mods.is_empty() {
                source.add_statement(EmptyLine::default());
            }
            for mod_name in &pub_mods {
                source.add_semi(format!(
                    "pub mod {}",
                    source_mod_name(&self.naming, *mod_name)
                ));
            }
        }

        Ok(source)
    }
}
