use clerr::{Code, PrimaryEntry, Report};
use lex::{ParseContext, Token};

use proto_packet_parse::SchemaFileTree;
use proto_packet_tree::{Import, SchemaFile, TypeDec, WithTypeName};

use crate::error::duplicate_decs;
use crate::InvalidSchemaFileError::*;
use crate::{
    validate_import, validate_type_dec, Error, ErrorInfo, InvalidImportError, InvalidTypeDecError,
    V_SCHEMA_FILE,
};

#[derive(Debug)]
pub enum InvalidSchemaFileError<'a> {
    InvalidImport(InvalidImportError<'a>),
    InvalidTypeDec(InvalidTypeDecError<'a>),
    DuplicateTypeDecNames { type_dec_names: Vec<Token<'a>> },
}

impl<'a> Error for InvalidSchemaFileError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        let code: &str = V_SCHEMA_FILE;
        match self {
            InvalidImport(e) => e.info(file_name, context),
            InvalidTypeDec(e) => e.info(file_name, context),
            DuplicateTypeDecNames {
                type_dec_names: type_names,
            } => duplicate_decs(file_name, context, code, "type dec names", type_names),
        }
    }
}

impl<'a> InvalidSchemaFileError<'a> {
    //! Report

    /// Creates the error report.
    pub fn report(&self, file_name: &str, context: ParseContext) -> Report {
        let info: ErrorInfo = self.info(file_name, context);
        let (code, header, info) = (info.code, info.header, info.info);
        let code: Code = Code::error(code, header);
        let primary: PrimaryEntry = PrimaryEntry::new(code).with_all_info(info);
        Report::new(primary)
    }
}

pub fn validate_schema_file<'a>(
    tree: &'a SchemaFileTree<'a>,
) -> Result<SchemaFile, InvalidSchemaFileError<'a>> {
    let mut schema_file: SchemaFile = SchemaFile::default();

    for import in &tree.imports {
        let import: Import = validate_import(import).map_err(|e| InvalidImport(e))?;
        debug_assert!(schema_file
            .get_import_by_effective_name(import.effective_name())
            .is_none());
        unsafe { schema_file.add_import(import) };
    }

    for type_dec in &tree.type_decs {
        let type_dec: TypeDec = validate_type_dec(type_dec).map_err(|e| InvalidTypeDec(e))?;

        if schema_file.get_dec_by_name(type_dec.type_name()).is_some() {
            return Err(DuplicateTypeDecNames {
                type_dec_names: tree.type_dec_name_tokens(type_dec.type_name()),
            });
        }

        unsafe { schema_file.add_type_dec(type_dec) }
    }

    Ok(schema_file)
}
