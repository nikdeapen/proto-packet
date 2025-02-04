use colored::ColoredString;

/// The info for a validation error.
#[derive(Debug)]
pub struct ErrorInfo {
    pub code: &'static str,       // The report error code.
    pub header: String,           // The report error code message.
    pub info: Vec<ColoredString>, // The extra primary entry info.
}

pub const V_TAG_NUMBER: &'static str = "V_TAG_NUMBER";
pub const V_SCHEMA_FILE: &'static str = "V_SCHEMA_FILE";
pub const V_MESSAGE: &'static str = "V_MESSAGE";
pub const V_MESSAGE_FIELD: &'static str = "V_MESSAGE_FIELD";
pub const V_ENUM: &'static str = "V_ENUM";
pub const V_ENUM_CASE: &'static str = "V_ENUM_CASE";
