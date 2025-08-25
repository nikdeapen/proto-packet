use crate::ParseSchemaFileError::*;
use crate::{
    parse_import, parse_type_dec, Error, ErrorInfo, ImportTree, ParseImportError,
    ParseTypeDecError, TypeDecTree, P_SCHEMA_FILE,
};
use clerr::{Code, Report, Severity};
use lex::{ParseContext, Token};

#[derive(Debug)]
pub struct SchemaFileTree<'a> {
    pub imports: Vec<ImportTree<'a>>,
    pub type_decs: Vec<TypeDecTree<'a>>,
}

impl<'a> SchemaFileTree<'a> {
    //! Type Declarations

    /// Gets the type name tokens with the given `name`.
    pub fn type_dec_name_tokens<S>(&self, name: S) -> Vec<Token<'_>>
    where
        S: AsRef<str>,
    {
        self.type_decs
            .iter()
            .map(|type_dec| type_dec.type_name_token())
            .filter(|token| token.value() == name.as_ref())
            .collect()
    }
}

#[derive(Debug)]
pub enum ParseSchemaFileError<'a> {
    InvalidImport(ParseImportError<'a>),
    InvalidTypeDeclaration(ParseTypeDecError<'a>),
    ExpectedTypeDeclaration,
}

impl<'a> Error for ParseSchemaFileError<'a> {
    fn info(&self, token: &str) -> ErrorInfo {
        match self {
            InvalidImport(e) => e.info(token),
            InvalidTypeDeclaration(e) => e.info(token),
            ExpectedTypeDeclaration => P_SCHEMA_FILE
                .expected_got_instead("the start of an import or type declaration", token),
        }
    }
}

impl<'a> ParseSchemaFileError<'a> {
    //! Report

    /// Converts the error to a report.
    pub fn to_report(self, file_name: &str, context: ParseContext, token: Token) -> Report {
        let info: ErrorInfo = self.info(token.value());
        let (code, header, message) = (info.code, info.header, info.message);
        let code: Code = Code::error(code, header);
        Report::new(code).with_token_info(
            file_name,
            token.line() + 1,
            token.position(),
            context
                .get_line_text(token.line())
                .unwrap_or("error: invalid report context"),
            token.value().chars().count(),
            Severity::Error,
            message.as_str(),
        )
    }
}

/// Parses a schema file.
///
/// This function parses the entire `ParseContext` and does not return any remaining context.
pub fn parse_schema_file(
    c: ParseContext,
) -> Result<SchemaFileTree, lex::Error<ParseSchemaFileError>> {
    let mut imports: Vec<ImportTree> = Vec::default();
    let mut type_decs: Vec<TypeDecTree> = Vec::default();
    let mut p: ParseContext = c;

    while let (Some(import), after_import) =
        parse_import(p).map_err(|e| lex::Error::new(e.token(), InvalidImport(e.to_error())))?
    {
        imports.push(import);
        p = after_import;
    }

    while let (Some(schema_dec), after_type_declaration) = parse_type_dec(p)
        .map_err(|e| lex::Error::new(e.token(), InvalidTypeDeclaration(e.to_error())))?
    {
        type_decs.push(schema_dec);
        p = after_type_declaration;
    }

    let (_white, after_white) = p.white_line_comments();
    if !after_white.is_empty() {
        Err(after_white.to_error(ExpectedTypeDeclaration))
    } else {
        Ok(SchemaFileTree { imports, type_decs })
    }
}
