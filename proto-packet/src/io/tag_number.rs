use std::fmt::{Debug, Display, Formatter};

/// A tag number.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TagNumber {
    tag_number: u32,
}

impl TagNumber {
    //! Validation

    /// Checks if `tag_number` is valid.
    #[inline(always)]
    pub fn is_valid(tag_number: u32) -> bool {
        tag_number != 0 && tag_number <= 0x7FFF_FFFF
    }
}

impl TagNumber {
    //! Construction

    /// Creates a new tag number.
    ///
    /// Returns `None` if the `tag_number` is invalid.
    pub fn new(tag_number: u32) -> Option<Self> {
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
    #[inline(always)]
    pub unsafe fn new_unchecked(tag_number: u32) -> Self {
        debug_assert!(Self::is_valid(tag_number));

        Self { tag_number }
    }
}

impl TagNumber {
    //! Properties

    /// Gets the tag number.
    pub fn tag_number(&self) -> u32 {
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
