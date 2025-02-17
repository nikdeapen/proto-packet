use lex::parse::IntParser;
use lex::{ParseContext, Token};

use proto_packet_parse::VariantTree;
use proto_packet_tree::{TypeName, Variant, VariantCase, WithCaseName, WithComments};

use crate::error::{duplicate_decs, invalid_name};
use crate::variant::variant::InvalidVariantReason::InvalidName;
use crate::InvalidVariantReason::{DuplicateCaseName, DuplicateCaseNumber, InvalidCase};
use crate::{validate_variant_case, Error, ErrorInfo, InvalidVariantCaseError};

#[derive(Debug)]
pub struct InvalidVariantError<'a> {
    pub variant_name: Token<'a>,
    pub reason: InvalidVariantReason<'a>,
}

#[derive(Debug)]
pub enum InvalidVariantReason<'a> {
    InvalidName { error_variant: &'static str },
    InvalidCase(InvalidVariantCaseError<'a>),
    DuplicateCaseName { case_names: Vec<Token<'a>> },
    DuplicateCaseNumber { tag_numbers: Vec<Token<'a>> },
}

impl<'a> Error for InvalidVariantError<'a> {
    fn info(&self, file_name: &str, context: ParseContext) -> ErrorInfo {
        let code: &str = "V_VARIANT";
        match &self.reason {
            InvalidName { error_variant } => invalid_name(
                file_name,
                context,
                "variant",
                self.variant_name,
                *error_variant,
                code,
            ),
            InvalidCase(e) => e.info(file_name, context),
            DuplicateCaseName { case_names } => {
                duplicate_decs(file_name, context, code, "variant names", case_names)
            }
            DuplicateCaseNumber { tag_numbers } => {
                duplicate_decs(file_name, context, code, "case numbers", tag_numbers)
            }
        }
    }
}

pub fn validate_variant<'a>(tree: &'a VariantTree<'a>) -> Result<Variant, InvalidVariantError<'a>> {
    let variant_name: TypeName =
        TypeName::new(tree.variant_name.value()).map_err(|error_variant| InvalidVariantError {
            variant_name: tree.variant_name,
            reason: InvalidName { error_variant },
        })?;
    let mut variant: Variant = Variant::from(variant_name);

    for comment in &tree.comments {
        variant.add_comment(comment.value().trim_end());
    }

    for case in &tree.cases {
        let case: VariantCase = validate_variant_case(&case).map_err(|e| InvalidVariantError {
            variant_name: tree.variant_name,
            reason: InvalidCase(e),
        })?;

        if variant.case_with_name(case.case_name()).is_some() {
            return Err(InvalidVariantError {
                variant_name: tree.variant_name,
                reason: DuplicateCaseName {
                    case_names: tree.case_name_tokens(case.case_name()),
                },
            });
        }

        if variant.case_with_number(case.tag_number()).is_some() {
            return Err(InvalidVariantError {
                variant_name: tree.variant_name,
                reason: DuplicateCaseNumber {
                    tag_numbers: tree.tag_number_tokens(case.tag_number(), &IntParser::default()),
                },
            });
        }

        unsafe { variant.add_case(case) }
    }

    Ok(variant)
}
