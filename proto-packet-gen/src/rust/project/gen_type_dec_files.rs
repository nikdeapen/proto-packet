use code_gen::{CodeBuffer, Source, Statement};
use file_storage::FilePath;

use proto_packet_tree::{ModName, ModPath, Project, TypeDec, WithTypeName};

use crate::rust::{Error, GenRust};
use crate::Generated;

impl GenRust {
    //! Gen Type Declaration Files

    /// Adds the type declarations files for the `project` to `generated`.
    pub(in crate::rust) fn gen_type_dec_files(
        &self,
        project: &Project,
        generated: &mut Generated,
    ) -> Result<(), Error> {
        for (mod_path, schema_file) in project.schemas() {
            for type_dec in schema_file.type_decs() {
                let source: Source = self.gen_type_dec(type_dec);
                let mod_name: ModName = self.naming.mod_name_for_type_name(type_dec.type_name())?;
                let mod_path: ModPath = mod_path.clone().with_appended(mod_name.to_ref());
                let file: FilePath = self
                    .naming
                    .file_naming
                    .file_for_mod_path(generated.target(), mod_path.to_ref())?;
                let mut b: CodeBuffer = CodeBuffer::default();
                source.write(&mut b, 0);
                generated.add_source(file, b.export());
            }
        }
        Ok(())
    }

    fn gen_type_dec(&self, type_dec: &TypeDec) -> Source {
        match type_dec {
            TypeDec::MessageDec(message) => self.gen_message(message),
        }
    }
}
