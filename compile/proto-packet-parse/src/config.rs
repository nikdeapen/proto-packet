/// Creates the parse config for schema files.
pub fn config() -> lex::Config {
    unsafe { lex::Config::default().with_line_comment_delimiter("//") }
}
