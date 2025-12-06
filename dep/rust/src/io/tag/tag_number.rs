use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// A tag number.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TagNumber {
    tag: u32,
}

impl TagNumber {
    //! Validation

    /// The maximum valid tag number. (2^31-1 = 2_147_483_647)
    pub const MAX_TAG_NUMBER: u32 = 0x7FFF_FFFF;

    /// Checks if the `tag` is valid.
    pub const fn is_valid(tag: u32) -> bool {
        tag > 0 && tag <= Self::MAX_TAG_NUMBER
    }
}

impl TagNumber {
    //! Construction

    /// Creates a new tag number.
    ///
    /// Returns `None` if the `tag` is invalid.
    pub const fn new(tag: u32) -> Option<Self> {
        if Self::is_valid(tag) {
            Some(Self { tag })
        } else {
            None
        }
    }

    /// Creates a new tag number.
    ///
    /// # Unsafe
    /// The `tag` must be valid.
    pub const unsafe fn new_unchecked(tag: u32) -> Self {
        debug_assert!(Self::is_valid(tag));

        Self { tag: tag }
    }
}

impl TagNumber {
    //! Properties

    /// Gets the tag number.
    pub const fn tag(&self) -> u32 {
        self.tag
    }
}

impl Debug for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for TagNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}
