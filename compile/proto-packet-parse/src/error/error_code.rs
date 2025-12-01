use crate::ErrorInfo;

/// An error code. `(code, header)`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ErrorCode(&'static str, &'static str);

impl ErrorCode {
    //! Error Info

    /// Gets the error info.
    pub fn info<S>(&self, message: S) -> ErrorInfo
    where
        S: Into<String>,
    {
        ErrorInfo {
            code: self.0,
            header: self.1,
            message: message.into(),
        }
    }

    /// Creates the `expected x got y instead` error info.
    pub fn expected_got_instead(&self, expected: &str, got_instead: &str) -> ErrorInfo {
        self.info(ErrorInfo::expected_got_instead(expected, got_instead))
    }
}

pub const P_SCHEMA_FILE: ErrorCode = ErrorCode("P_SCHEMA_FILE", "invalid schema file");
pub const P_IMPORT: ErrorCode = ErrorCode("P_IMPORT", "invalid schema declaration");

pub const P_TAG_NUMBER: ErrorCode = ErrorCode("P_TAG_NUMBER", "invalid tag number");
pub const P_TYPE_TAG: ErrorCode = ErrorCode("P_TYPE_TAG", "invalid type tag");

pub const P_STRUCT: ErrorCode = ErrorCode("P_STRUCT", "invalid struct declaration");
pub const P_STRUCT_FIELD: ErrorCode =
    ErrorCode("P_STRUCT_FIELD", "invalid struct field declaration");

pub const P_MESSAGE: ErrorCode = ErrorCode("P_MESSAGE", "invalid message declaration");
pub const P_MESSAGE_FIELD: ErrorCode =
    ErrorCode("P_MESSAGE_FIELD", "invalid message field declaration");

pub const P_ENUM: ErrorCode = ErrorCode("P_ENUM", "invalid enum declaration");
pub const P_ENUM_CASE: ErrorCode = ErrorCode("P_ENUM_CASE", "invalid enum case declaration");

pub const P_VARIANT: ErrorCode = ErrorCode("P_VARIANT", "invalid variant declaration");
pub const P_VARIANT_CASE: ErrorCode =
    ErrorCode("P_VARIANT_CASE", "invalid variant case declaration");
