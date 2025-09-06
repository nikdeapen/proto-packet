use crate::rust::Naming;
use std::collections::HashSet;
use std::sync::OnceLock;

// The keywords.
static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

impl Naming {
    //! Keywords

    /// Gets the keywords.
    pub fn keywords() -> &'static HashSet<&'static str> {
        KEYWORDS.get_or_init(|| {
            include_str!("keywords.txt")
                .split("\n")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect()
        })
    }

    /// Checks if `s` is a keyword.
    pub fn is_keyword<S>(&self, s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::keywords().contains(s.as_ref())
    }
}
