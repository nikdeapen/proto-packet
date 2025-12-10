use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// A tag number.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TagNumber {
    value: u32,
}

impl TagNumber {
    //! Validation

    /// The maximum valid tag number. (2^31-1 = 2_147_483_647)
    pub const MAX_TAG_NUMBER: u32 = 0x7FFF_FFFF;

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
            Some(Self { value: value })
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
    pub const fn value(&self) -> u32 {
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
