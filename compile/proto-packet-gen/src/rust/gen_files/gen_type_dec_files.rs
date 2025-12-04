use crate::rust::{GenRust, ModTree};
use crate::{Reader, Writer};
use clerr::Report;
use code_gen::Source;
use proto_packet_tree::{ModPath, TypeDec, WithTypeName};

impl GenRust {
    //! Gen Files: Type Declarations

    /// Generates the type declaration files.
    pub(in crate::rust) fn gen_type_dec_files<R, W>(
        &self,
        schemas: &[R],
        writer: &W,
    ) -> Result<ModTree, Report>
    where
        R: Reader,
        W: Writer,
    {
        let mut mods: ModTree = ModTree::default();
        for schema in schemas {
            for type_dec in schema.read()?.type_decs() {
                let source: Source = match type_dec {
                    TypeDec::StructDec(s) => self.gen_struct(schema.mod_path(), &s),
                    TypeDec::MessageDec(m) => self.gen_message(schema.mod_path(), &m),
                    TypeDec::EnumDec(e) => self.gen_enum(e),
                };
                let mod_path: ModPath = schema.mod_path().with_appended(
                    self.naming
                        .mod_name_for_type_name(type_dec.type_name())
                        .map_err(|e| e.to_report())?
                        .to_ref(),
                );
                writer.write(
                    &source,
                    self.naming
                        .file_name_for_mod_path(mod_path.to_ref())?
                        .as_str(),
                )?;
                mods.insert(&self.naming, schema.mod_path(), type_dec.type_name())?;
            }
        }
        Ok(mods)
    }
}
