use crate::config::GenConfig;
use crate::rust::{Error, FileNaming, Naming, Typing};
use crate::{CodeGenerator, GeneratedCode};
use clerr::Report;
use code_gen::{CodeBuffer, Source, Statement};
use file_storage::{FilePath, FolderPath};
use proto_packet_tree::{ModName, ModPath, ModPathRef, Project, TypeDec, WithTypeName};

/// Responsible for generating code for Rust projects.
#[derive(Debug, Default)]
pub struct GenRust {
    pub(in crate::rust) config: GenConfig,
    pub(in crate::rust) naming: Naming,
    pub(in crate::rust) typing: Typing,
}

impl GenRust {
    //! Construction

    /// Creates a new generator.
    pub fn new(config: GenConfig) -> Self {
        Self {
            config,
            naming: Naming::default(),
            typing: Typing::default(),
        }
    }
}

impl GenRust {
    //! Type Decs

    fn gen_type_dec_files(
        &self,
        project: &Project,
        generated: &mut GeneratedCode,
    ) -> Result<(), Error> {
        for (mod_path, schema_file) in project.schema_files() {
            for type_dec in schema_file.type_decs() {
                let source: Source = self.gen_type_dec(mod_path.to_ref(), type_dec);
                let mod_name: ModName = unsafe {
                    ModName::new_unchecked(self.naming.mod_name_for_type_name(type_dec.type_name()))
                };
                let mod_path: ModPath = mod_path.clone().with_appended(mod_name.to_ref());
                // todo -- make configurable
                let file: FilePath = FileNaming::default()
                    .file_for_mod_path(mod_path.to_ref(), generated.target())?;
                let mut b: CodeBuffer = CodeBuffer::default();
                source.write(&mut b, 0);
                unsafe { generated.add_source(file, b.into()) };
            }
        }
        Ok(())
    }

    fn gen_type_dec(&self, mod_path: ModPathRef, type_dec: &TypeDec) -> Source {
        match type_dec {
            TypeDec::StructDec(structure) => self.gen_struct(mod_path, structure),
            TypeDec::MessageDec(message) => self.gen_message(mod_path, message),
            TypeDec::EnumDec(enom) => self.gen_enum(mod_path, enom),
            TypeDec::VariantDec(variant) => self.gen_variant(mod_path, variant),
        }
    }
}

impl CodeGenerator for GenRust {
    fn generate(&self, project: &Project, target: &FolderPath) -> Result<GeneratedCode, Report> {
        let mut generated: GeneratedCode = GeneratedCode::from(target.clone());

        self.gen_mod_files(project, &mut generated)
            .map_err(|e| e.to_report())?;
        self.gen_type_dec_files(project, &mut generated)
            .map_err(|e| e.to_report())?;

        Ok(generated)
    }
}
