use crate::{Error, Resolver, TypeLinker};
use proto_packet_tree::{Import, ModPathRef, SchemaFile, TypeDec, TypeNameRef, WithTypeName};

/// Responsible for linking schema files.
#[derive(Debug)]
pub struct SchemaLinker<'a> {
    mod_path: ModPathRef<'a>,
}

impl<'a> SchemaLinker<'a> {
    //! Construction

    /// Creates a new schema linker.
    ///
    /// # Safety
    /// The `mod_path` must not be empty.
    pub unsafe fn new_unchecked(mod_path: ModPathRef<'a>) -> Self {
        debug_assert!(!mod_path.is_empty());

        Self { mod_path }
    }
}

impl<'a> SchemaLinker<'a> {
    //! Link

    /// Links the `schema_file`.
    pub fn link<'b>(&'a self, schema_file: &'b SchemaFile) -> Result<SchemaFile, Error> {
        let local_names: Vec<TypeNameRef> = schema_file
            .type_decs()
            .iter()
            .map(|t| t.type_name())
            .collect();
        let imports: &'b [Import] = schema_file.imports();
        let resolver: Resolver =
            unsafe { Resolver::new_unchecked(self.mod_path, local_names.as_slice(), imports) };
        let linker: TypeLinker = TypeLinker::from(resolver);

        let mut result: SchemaFile = SchemaFile::default();
        for type_dec in schema_file.type_decs() {
            let type_dec: TypeDec = linker.link(type_dec)?;
            debug_assert!(result.can_add_type_dec(&type_dec));
            unsafe { result.add_type_dec(type_dec) };
        }
        Ok(result)
    }
}
