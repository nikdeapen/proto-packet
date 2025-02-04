use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet_parse::EnumTree;
use proto_packet_tree::{Enum, EnumCase, TypeName, WithCaseName, WithComments, WithTagNumber};

use crate::error::{duplicate_decs, invalid_name};
use crate::InvalidEnumReason::{DuplicateCaseName, DuplicateCaseNumber, InvalidCase, InvalidName};
use crate::{validate_enum_case, Error, ErrorInfo, InvalidEnumCaseError, V_ENUM};

#[derive(Debug)]
pub struct InvalidEnumError<'a> {
    pub enum_name: Token<'a>,
    pub reason: InvalidEnumReason<'a>,
}

#[derive(Debug)]
pub enum InvalidEnumReason<'a> {
    InvalidName { error_enum: &'static str },
    InvalidCase(InvalidEnumCaseError<'a>),
    DuplicateCaseName { case_names: Vec<Token<'a>> },
    DuplicateCaseNumber { tag_numbers: Vec<Token<'a>> },
}

impl<'a> Error for InvalidEnumError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        match &self.reason {
            InvalidName { error_enum } => invalid_name(
                file_name,
                context,
                "enum",
                self.enum_name,
                *error_enum,
                V_ENUM,
            ),
            InvalidCase(e) => e.info(file_name, context),
            DuplicateCaseName { case_names } => {
                duplicate_decs(file_name, context, V_ENUM, "enum names", case_names)
            }
            DuplicateCaseNumber { tag_numbers } => {
                duplicate_decs(file_name, context, V_ENUM, "case numbers", tag_numbers)
            }
        }
    }
}

pub fn validate_enum<'a>(tree: &'a EnumTree<'a>) -> Result<Enum, InvalidEnumError<'a>> {
    let enum_name: TypeName =
        TypeName::new(tree.enum_name.value()).map_err(|error_enum| InvalidEnumError {
            enum_name: tree.enum_name,
            reason: InvalidName { error_enum },
        })?;
    let mut enom: Enum = Enum::from(enum_name);

    for comment in &tree.comments {
        enom.add_comment(comment.value().trim_end());
    }

    for case in &tree.cases {
        let case: EnumCase = validate_enum_case(&case).map_err(|e| InvalidEnumError {
            enum_name: tree.enum_name,
            reason: InvalidCase(e),
        })?;

        if enom.case_with_name(case.case_name()).is_some() {
            return Err(InvalidEnumError {
                enum_name: tree.enum_name,
                reason: DuplicateCaseName {
                    case_names: tree.case_name_tokens(case.case_name()),
                },
            });
        }

        if enom.case_with_number(case.tag_number()).is_some() {
            return Err(InvalidEnumError {
                enum_name: tree.enum_name,
                reason: DuplicateCaseNumber {
                    tag_numbers: tree.tag_number_tokens(case.tag_number(), &IntParser::default()),
                },
            });
        }

        unsafe { enom.add_case(case) }
    }

    Ok(enom)
}
