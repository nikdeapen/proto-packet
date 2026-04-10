use std::fmt::{Debug, Display, Formatter};

/// A tag number.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TagNumber {
    value: u32,
}

impl TagNumber {
    //! Validation

    /// The maximum valid tag number. (2^15-1 = 32,767)
    pub const MAX_TAG_NUMBER: u32 = 0x0000_7FFF;

    /// Checks if the `value` is valid.
    pub const fn is_valid(value: u32) -> bool {
        value > 0 && value <= Self::MAX_TAG_NUMBER
    }
}

impl TagNumber {
    //! Construction

    /// Creates a new tag number.
    ///
    /// Returns `None` if the `value` is invalid.
    pub const fn new(value: u32) -> Option<Self> {
        if Self::is_valid(value) {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Creates a new tag number.
    ///
    /// # Safety
    /// The `value` must be valid.
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        debug_assert!(Self::is_valid(value));

        Self { value }
    }
}

impl TagNumber {
    //! Properties

    /// Gets the tag number.
    pub const fn value(self) -> u32 {
        self.value
    }
}

impl Debug for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::TagNumber;

    #[test]
    fn is_valid() {
        let cases: &[(u32, bool)] = &[
            (0, false),
            (1, true),
            (42, true),
            (TagNumber::MAX_TAG_NUMBER, true),
            (TagNumber::MAX_TAG_NUMBER + 1, false),
            (u32::MAX, false),
        ];
        for (value, expected) in cases {
            assert_eq!(TagNumber::is_valid(*value), *expected, "value={value}");
        }
    }

    #[test]
    fn new() {
        let cases: &[(u32, Option<u32>)] = &[
            (0, None),
            (1, Some(1)),
            (42, Some(42)),
            (TagNumber::MAX_TAG_NUMBER, Some(TagNumber::MAX_TAG_NUMBER)),
            (TagNumber::MAX_TAG_NUMBER + 1, None),
            (u32::MAX, None),
        ];
        for (input, expected) in cases {
            let actual: Option<u32> = TagNumber::new(*input).map(TagNumber::value);
            assert_eq!(actual, *expected, "input={input}");
        }
    }

    #[test]
    fn new_unchecked() {
        let cases: &[u32] = &[1, 42, TagNumber::MAX_TAG_NUMBER];
        for input in cases {
            let tag: TagNumber = unsafe { TagNumber::new_unchecked(*input) };
            assert_eq!(tag.value(), *input, "input={input}");
        }
    }

    #[test]
    fn value() {
        let cases: &[u32] = &[1, 42, 12345, TagNumber::MAX_TAG_NUMBER];
        for input in cases {
            let tag: TagNumber = TagNumber::new(*input).unwrap();
            assert_eq!(tag.value(), *input, "input={input}");
        }
    }
}
