use crate::InvalidEnumReason::{DuplicateCaseName, DuplicateCaseNumber, InvalidCase, InvalidName};
use crate::{validate_enum_case, Error, ErrorInfo, InvalidEnumCaseError, V_ENUM};
use lex::parse::IntParser;
use lex::{Context, Token};
use proto_packet_parse::EnumTree;
use proto_packet_tree::{Enum, EnumCase, TypeName, WithCaseName, WithComments, WithTagNumber};

#[derive(Debug)]
pub struct InvalidEnumError<'a> {
    pub enum_name: Token<'a>,
    pub reason: InvalidEnumReason<'a>,
}

#[derive(Debug)]
pub enum InvalidEnumReason<'a> {
    InvalidName { error: &'static str },
    InvalidCase(InvalidEnumCaseError<'a>),
    DuplicateCaseName { case_names: Vec<Token<'a>> },
    DuplicateCaseNumber { tags: Vec<Token<'a>> },
}

impl<'a> Error for InvalidEnumError<'a> {
    fn info(&self, file_name: &str, context: Context) -> ErrorInfo {
        match &self.reason {
            InvalidName { error } => {
                V_ENUM.invalid_name(file_name, context, "enum", self.enum_name, *error)
            }
            InvalidCase(e) => e.info(file_name, context),
            DuplicateCaseName { case_names } => {
                V_ENUM.duplicate_decs(file_name, context, "enum names", case_names)
            }
            DuplicateCaseNumber { tags } => {
                V_ENUM.duplicate_decs(file_name, context, "case numbers", tags)
            }
        }
    }
}

pub fn validate_enum<'a>(tree: &'a EnumTree<'a>) -> Result<Enum, InvalidEnumError<'a>> {
    let enum_name: TypeName =
        TypeName::new(tree.enum_name.value()).map_err(|error| InvalidEnumError {
            enum_name: tree.enum_name,
            reason: InvalidName { error },
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

        if enom.case_with_tag(case.tag()).is_some() {
            return Err(InvalidEnumError {
                enum_name: tree.enum_name,
                reason: DuplicateCaseNumber {
                    tags: tree.tag_number_tokens(case.tag(), &IntParser::default()),
                },
            });
        }

        unsafe { enom.add_case(case) }
    }

    Ok(enom)
}
