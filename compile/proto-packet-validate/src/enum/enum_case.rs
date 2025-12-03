use lex::{Context, Token};

use proto_packet::io::TagNumber;
use proto_packet_parse::EnumCaseTree;
use proto_packet_tree::{CaseName, EnumCase, WithComments};

use crate::InvalidEnumCaseReason::*;
use crate::{validate_tag_number, Error, ErrorInfo, InvalidTagNumberError, V_ENUM_CASE};

#[derive(Debug)]
pub struct InvalidEnumCaseError<'a> {
    pub case_name: Token<'a>,
    pub reason: InvalidEnumCaseReason<'a>,
}

#[derive(Debug)]
pub enum InvalidEnumCaseReason<'a> {
    InvalidCaseName { error: &'static str },
    InvalidTagNumber(InvalidTagNumberError<'a>),
}

impl<'a> Error for InvalidEnumCaseError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match &self.reason {
            InvalidCaseName { error } => {
                V_ENUM_CASE.invalid_name(file_name, context, "enum case", self.case_name, *error)
            }
            InvalidTagNumber(e) => e.info(file_name, context),
        }
    }
}

pub fn validate_enum_case<'a>(
    tree: &'a EnumCaseTree,
) -> Result<EnumCase, InvalidEnumCaseError<'a>> {
    let case_name: CaseName =
        CaseName::new(tree.case_name.value()).map_err(|error_enum| InvalidEnumCaseError {
            case_name: tree.case_name,
            reason: InvalidCaseName { error: error_enum },
        })?;
    let tag_number: TagNumber =
        validate_tag_number(tree.tag_number).map_err(|e| InvalidEnumCaseError {
            case_name: tree.case_name,
            reason: InvalidTagNumber(e),
        })?;

    let mut case: EnumCase = EnumCase::new(case_name, tag_number);
    for comment in &tree.comments {
        case.add_comment(comment.value());
    }

    Ok(case)
}
