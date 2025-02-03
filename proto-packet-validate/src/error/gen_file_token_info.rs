use clerr::text_file::TokenInfo;
use clerr::Severity;
use colored::ColoredString;
use lex::{ParseContext, Token};

pub fn gen_file_token_info(
    file_name: &str,
    c: ParseContext,
    t: Token,
    message: &str,
) -> Vec<ColoredString> {
    let mut info: Vec<ColoredString> = Vec::default();
    add_file_token_info(file_name, c, t, message, &mut info);
    info
}

pub fn add_file_token_info(
    file_name: &str,
    c: ParseContext,
    t: Token,
    message: &str,
    info: &mut Vec<ColoredString>,
) {
    let mut i: Vec<ColoredString> = TokenInfo {
        file_name,
        line: t.line() + 1,
        position: t.position(),
        len: t.value().chars().count(),
        line_text: c
            .get_line_text(t.line())
            .unwrap_or("FATAL: could not resolve source line text"),
        message,
        severity: Severity::Error,
    }
    .into();
    i.drain(..).for_each(|s| info.push(s))
}
