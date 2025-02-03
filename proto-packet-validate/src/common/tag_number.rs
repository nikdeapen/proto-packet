use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet::io::TagNumber;

use crate::InvalidTagNumberReason::{InvalidNumberFormat, OutOfRange, Zero};
use crate::{gen_file_token_info, Error, ErrorInfo, V_TAG_NUMBER};

#[derive(Debug)]
pub struct InvalidTagNumberError<'a> {
    pub tag_number: Token<'a>,
    pub reason: InvalidTagNumberReason,
}

#[derive(Debug)]
pub enum InvalidTagNumberReason {
    Zero,
    OutOfRange,
    InvalidNumberFormat { error_message: &'static str },
}

impl<'a> Error for InvalidTagNumberError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        ErrorInfo {
            code: V_TAG_NUMBER,
            header: match &self.reason {
                Zero => "tag numbers cannot be '0'".to_string(),
                OutOfRange => "tag numbers must be in the range [1, 2_147_483_647]".to_string(),
                InvalidNumberFormat { error_message } => {
                    format!("invalid tag number format: {}", error_message)
                }
            },
            info: gen_file_token_info(file_name, context, self.tag_number, "declared here"),
        }
    }
}

/// Validates the tag number.
pub fn validate_tag_number(tag_number: Token) -> Result<TagNumber, InvalidTagNumberError> {
    let parser: IntParser = IntParser::default();
    let value: u32 = parser
        .parse_u32(tag_number.value())
        .map_err(|e| InvalidTagNumberError {
            tag_number,
            reason: InvalidNumberFormat {
                error_message: e.message(),
            },
        })?;
    if value == 0 {
        Err(InvalidTagNumberError {
            tag_number,
            reason: Zero,
        })
    } else if !TagNumber::is_valid(value) {
        Err(InvalidTagNumberError {
            tag_number,
            reason: OutOfRange,
        })
    } else {
        Ok(unsafe { TagNumber::new_unchecked(value) })
    }
}
