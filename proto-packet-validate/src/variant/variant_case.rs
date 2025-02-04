use lex::{ParseContext, Token};

use proto_packet::io::TagNumber;
use proto_packet_parse::VariantCaseTree;
use proto_packet_tree::{CaseName, TypeTag, VariantCase, WithComments};

use crate::InvalidVariantCaseReason::*;
use crate::{
    validate_tag_number, validate_type_tag, Error, ErrorInfo, InvalidTagNumberError,
    InvalidTypeTagError,
};

#[derive(Debug)]
pub struct InvalidVariantCaseError<'a> {
    pub case_name: Token<'a>,
    pub reason: InvalidVariantCaseReason<'a>,
}

#[derive(Debug)]
pub enum InvalidVariantCaseReason<'a> {
    InvalidCaseName { error_message: &'static str },
    InvalidTypeTag(InvalidTypeTagError<'a>),
    InvalidTagNumber(InvalidTagNumberError<'a>),
}

impl<'a> Error for InvalidVariantCaseError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        let code: &str = "V_VARIANT_CASE";
        match &self.reason {
            InvalidCaseName { error_message } => crate::invalid_name(
                file_name,
                context,
                "variant case",
                self.case_name,
                *error_message,
                code,
            ),
            InvalidTypeTag(e) => e.info(file_name, context),
            InvalidTagNumber(e) => e.info(file_name, context),
        }
    }
}
pub fn validate_variant_case<'a>(
    tree: &'a VariantCaseTree,
) -> Result<VariantCase, InvalidVariantCaseError<'a>> {
    let case_name: CaseName =
        CaseName::new(tree.case_name.value()).map_err(|error_message| InvalidVariantCaseError {
            case_name: tree.case_name,
            reason: InvalidCaseName { error_message },
        })?;
    let type_tag: TypeTag =
        validate_type_tag(&tree.type_tag).map_err(|e| InvalidVariantCaseError {
            case_name: tree.case_name,
            reason: InvalidTypeTag(e),
        })?;
    let tag_number: TagNumber =
        validate_tag_number(tree.tag_number).map_err(|e| InvalidVariantCaseError {
            case_name: tree.case_name,
            reason: InvalidTagNumber(e),
        })?;

    let mut case: VariantCase = VariantCase::new(case_name, type_tag, tag_number);
    for comment in &tree.comments {
        case.add_comment(comment.value());
    }

    Ok(case)
}
