use lex::{Context, Token};
use proto_packet_tree::QualifiedName;

/// Parses an optional qualified name. (ex: `mod.path.TypeName`)
///
/// Returns `(Some(qualified_name), after_qualified_name)`.
/// Returns `(None, c)` when the next token is not a qualified name.
pub fn parse_qualified_name(c: Context) -> (Option<Token>, Context) {
    if let (Some(name), after_name) = unsafe { c.match_prefix(|b| QualifiedName::is_valid_byte(b)) }
    {
        (Some(name.token()), after_name)
    } else {
        (None, c)
    }
}
