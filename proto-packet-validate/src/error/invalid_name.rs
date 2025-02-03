use colored::ColoredString;
use lex::{ParseContext, Token};

use crate::{gen_file_token_info, ErrorInfo};

pub fn invalid_name(
    file_name: &str,
    context: ParseContext,
    name_type: &str,
    name: Token,
    message: &str,
    code: &'static str,
) -> ErrorInfo {
    let info: Vec<ColoredString> = gen_file_token_info(file_name, context, name, message);
    ErrorInfo {
        code,
        header: format!("invalid {} name", name_type),
        info,
    }
}
