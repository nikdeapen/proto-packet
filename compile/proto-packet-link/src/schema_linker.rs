use std::collections::HashMap;

use proto_packet_tree::{Import, ModPathRef, SchemaFile, TypeDec, TypeNameRef, WithTypeName};

use crate::{Error, TypeLinker};

/// Responsible for linking schema files.
#[derive(Debug)]
pub struct SchemaLinker<'a> {
    mod_path: ModPathRef<'a>,
    all_names: &'a HashMap<ModPathRef<'a>, Vec<TypeNameRef<'a>>>,
}

impl<'a> SchemaLinker<'a> {
    //! Construction

    /// Creates a new schema linker.
    ///
    /// # Unsafe
    /// todo -- validation
    pub unsafe fn new(
        mod_path: ModPathRef<'a>,
        all_names: &'a HashMap<ModPathRef<'a>, Vec<TypeNameRef<'a>>>,
    ) -> Self {
        Self {
            mod_path,
            all_names,
        }
    }
}

impl<'a> SchemaLinker<'a> {
    //! Link

    /// Links the `project`.
    pub fn link<'b>(&'a self, schema_file: &'b SchemaFile) -> Result<SchemaFile, Error> {
        let local_names: Vec<TypeNameRef> = schema_file
            .type_decs()
            .iter()
            .map(|t| t.type_name())
            .collect();
        let imports: &'b [Import] = schema_file.imports();
        let type_linker: TypeLinker = unsafe {
            TypeLinker::new(
                self.mod_path,
                local_names.as_slice(),
                imports,
                self.all_names,
            )
        };
        let type_decs = schema_file.type_decs();
        let mut schema_file: SchemaFile = SchemaFile::default();
        for type_dec in type_decs {
            let type_dec: TypeDec = type_linker.link(type_dec)?;
            unsafe { schema_file.add_type_dec(type_dec) };
        }
        Ok(schema_file)
    }
}
