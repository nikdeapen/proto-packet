use crate::rust::Naming;
use std::str::Chars;

impl Naming {
    //! Utilities

    /// Converts the `pascal_case` string to a snake case string.
    pub fn pascal_to_snake_case(&self, pascal_case: &str) -> String {
        let mut snake_case: String = String::new();
        for (i, c) in pascal_case.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        }
        snake_case
    }

    /// Converts the `snake_case` string to a pascal case string.
    pub fn snake_to_pascal_case(&self, snake_case: &str) -> String {
        let mut pascal_case: String = String::with_capacity(snake_case.len());
        for segment in snake_case.split("_") {
            let mut chars: Chars = segment.chars();
            if let Some(first) = chars.next() {
                pascal_case.push(first.to_ascii_uppercase());
                pascal_case.push_str(&segment[first.len_utf8()..]);
            }
        }
        pascal_case
    }
}
