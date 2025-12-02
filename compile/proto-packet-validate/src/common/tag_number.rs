use crate::InvalidTagNumberReason::{InvalidNumberFormat, OutOfRange, Zero};
use crate::{Error, ErrorInfo, V_TAG_NUMBER};
use lex::parse::IntParser;
use lex::{Context, Token};
use proto_packet::io::TagNumber;

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
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        let (header, message) = match self.reason {
            Zero => ("tag numbers cannot be zero".to_string(), "declared here"),
            OutOfRange => (
                format!(
                    "tag number out of range: valid-range=[{}, {}]",
                    1,
                    TagNumber::MAX_TAG_NUMBER
                ),
                "declared here",
            ),
            InvalidNumberFormat { error_message } => {
                ("invalid tag number".to_string(), error_message)
            }
        };
        V_TAG_NUMBER
            .info(header)
            .with_token_info(file_name, context, self.tag_number, message)
    }
}

pub fn validate_tag_number(token: Token) -> Result<TagNumber, InvalidTagNumberError> {
    let parser: IntParser = IntParser::default();
    let value: u32 = parser
        .parse_u32(token.value())
        .map_err(|e| InvalidTagNumberError {
            tag_number: token,
            reason: InvalidNumberFormat {
                error_message: e.message(),
            },
        })?;
    if value == 0 {
        Err(InvalidTagNumberError {
            tag_number: token,
            reason: Zero,
        })
    } else if !TagNumber::is_valid(value) {
        Err(InvalidTagNumberError {
            tag_number: token,
            reason: OutOfRange,
        })
    } else {
        Ok(unsafe { TagNumber::new_unchecked(value) })
    }
}
