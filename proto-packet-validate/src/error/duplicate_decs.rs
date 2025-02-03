use colored::ColoredString;
use lex::{ParseContext, Token};

use crate::{add_file_token_info, ErrorInfo};

pub fn duplicate_decs(
    file_name: &str,
    c: ParseContext,
    code: &'static str,
    declaration_type: &str,
    declarations: &[Token],
) -> ErrorInfo {
    let header: String = format!("duplicate {}", declaration_type);
    let mut info: Vec<ColoredString> = Vec::default();
    for declaration in declarations {
        add_file_token_info(file_name, c, *declaration, "declared here", &mut info);
    }
    ErrorInfo { code, header, info }
}
