use crate::ErrorInfo;
use lex::{Context, Token};

/// An error code.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    //! Code

    /// Gets the error info.
    pub fn info<S>(&self, header: S) -> ErrorInfo
    where
        S: Into<String>,
    {
        ErrorInfo {
            code: self.0,
            header: header.into(),
            entries: vec![],
        }
    }

    /// Gets the error info for an invalid name.
    pub fn invalid_name(
        &self,
        file_name: &str,
        context: Context,
        name_type: &str,
        name: Token,
        message: &str,
    ) -> ErrorInfo {
        self.info(format!("invalid {} name", name_type))
            .with_token_info(file_name, context, name, message)
    }

    /// Gets the error info for duplicate declaration names.
    pub fn duplicate_decs(
        &self,
        file_name: &str,
        context: Context,
        dec_type: &str,
        dec_names: &[Token],
    ) -> ErrorInfo {
        dec_names.iter().fold(
            self.info(format!("duplicate {} declarations", dec_type)),
            |info, declaration| {
                info.with_token_info(file_name, context, *declaration, "declared here")
            },
        )
    }
}

pub const V_ENUM: ErrorCode = ErrorCode("V_ENUM");
pub const V_ENUM_CASE: ErrorCode = ErrorCode("V_ENUM_CASE");
pub const V_IMPORT: ErrorCode = ErrorCode("V_IMPORT");
pub const V_MESSAGE: ErrorCode = ErrorCode("V_MESSAGE");
pub const V_MESSAGE_FIELD: ErrorCode = ErrorCode("V_MESSAGE_FIELD");
pub const V_SCHEMA_FILE: ErrorCode = ErrorCode("V_SCHEMA_FILE");
pub const V_STRUCT: ErrorCode = ErrorCode("V_STRUCT");
pub const V_STRUCT_FIELD: ErrorCode = ErrorCode("V_STRUCT_FIELD");
pub const V_TAG_NUMBER: ErrorCode = ErrorCode("V_TAG_NUMBER");
pub const V_TYPE_TAG: ErrorCode = ErrorCode("V_TYPE_TAG");
pub const V_VARIANT: ErrorCode = ErrorCode("V_VARIANT");
pub const V_VARIANT_CASE: ErrorCode = ErrorCode("V_VARIANT_CASE");
pub const V_SERVICE: ErrorCode = ErrorCode("V_SERVICE");
pub const V_SERVICE_CALL: ErrorCode = ErrorCode("V_SERVICE_CALL");
