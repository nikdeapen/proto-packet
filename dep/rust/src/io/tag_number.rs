use std::fmt::{Debug, Display, Formatter};

/// A tag number.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TagNumber {
    tag_number: u32,
}

impl TagNumber {
    //! Validation

    /// The maximum valid tag number.
    pub const MAX_TAG_NUMBER: u32 = 0x7FFF_FFFF;

    /// Checks if `tag_number` is valid.
    pub const fn is_valid(tag_number: u32) -> bool {
        tag_number != 0 && tag_number <= Self::MAX_TAG_NUMBER
    }
}

impl TagNumber {
    //! Construction

    /// Creates a new tag number.
    ///
    /// Returns `None` if the `tag_number` is invalid.
    pub const fn new(tag_number: u32) -> Option<Self> {
        if Self::is_valid(tag_number) {
            Some(Self { tag_number })
        } else {
            None
        }
    }

    /// Creates a new tag number.
    ///
    /// # Unsafe
    /// The `tag_number` must be valid.
    pub const unsafe fn new_unchecked(tag_number: u32) -> Self {
        debug_assert!(Self::is_valid(tag_number));

        Self { tag_number }
    }
}

impl TagNumber {
    //! Properties

    /// Gets the tag number.
    pub const fn tag_number(&self) -> u32 {
        self.tag_number
    }
}

impl Debug for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag_number)
    }
}
