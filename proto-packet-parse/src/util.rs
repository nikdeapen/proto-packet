use lex::{CommentConfig, Config};

/// Creates the parse config.
pub fn create_parse_config() -> Config {
    unsafe {
        Config::default()
            .with_comment_config(CommentConfig::default().with_line_comment_delimiter("//"))
    }
}
