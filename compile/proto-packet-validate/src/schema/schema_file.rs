use crate::InvalidSchemaFileError::*;
use crate::{
    validate_import, validate_type_dec, Error, ErrorInfo, InvalidImportError, InvalidTypeDecError,
    V_SCHEMA_FILE,
};
use clerr::{Code, Report};
use colored::ColoredString;
use lex::{Context, Token};
use proto_packet_parse::SchemaFileTree;
use proto_packet_tree::{Import, SchemaFile, TypeDec, WithTypeName};

#[derive(Debug)]
pub enum InvalidSchemaFileError<'a> {
    InvalidImport(InvalidImportError<'a>),
    InvalidTypeDec(InvalidTypeDecError<'a>),
    DuplicateTypeDecNames { type_dec_names: Vec<Token<'a>> },
}

impl<'a> Error for InvalidSchemaFileError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match self {
            InvalidImport(e) => e.info(file_name, context),
            InvalidTypeDec(e) => e.info(file_name, context),
            DuplicateTypeDecNames {
                type_dec_names: type_names,
            } => V_SCHEMA_FILE.duplicate_decs(file_name, context, "type dec names", type_names),
        }
    }
}

impl<'a> InvalidSchemaFileError<'a> {
    //! Report

    /// Creates the error report.
    pub fn to_report(&self, file_name: &str, context: Context) -> Report {
        let info: ErrorInfo = self.info(file_name, context);
        let (code, header, mut info) = (info.code, info.header, info.entries);
        let code: Code = Code::error(code, header);
        let info: Vec<ColoredString> = info.iter_mut().flat_map(|e| e.drain(..)).collect();
        Report::new(code).with_entry(info)
    }
}

pub fn validate_schema_file<'a>(
    tree: &'a SchemaFileTree<'a>,
) -> Result<SchemaFile, InvalidSchemaFileError<'a>> {
    let mut schema_file: SchemaFile = SchemaFile::default();

    for import in &tree.imports {
        let import: Import = validate_import(import).map_err(|e| InvalidImport(e))?;

        if !schema_file.can_add_import(&import) {
            return Err(DuplicateTypeDecNames {
                type_dec_names: tree.type_dec_name_tokens(import.effective_name()),
            });
        }

        unsafe { schema_file.add_import(import) };
    }

    for type_dec in &tree.type_decs {
        let type_dec: TypeDec = validate_type_dec(type_dec).map_err(|e| InvalidTypeDec(e))?;

        if !schema_file.can_add_type_dec(&type_dec) {
            return Err(DuplicateTypeDecNames {
                type_dec_names: tree.type_dec_name_tokens(type_dec.type_name()),
            });
        }

        unsafe { schema_file.add_type_dec(type_dec) }
    }
    // todo -- handle matched type-dec/import names

    Ok(schema_file)
}
