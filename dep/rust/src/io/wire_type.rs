use std::fmt::{Debug, Display, Formatter};

use crate::io::WireType::*;

/// A wire type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WireType {
    /// A fixed 1-byte of data.
    Fixed1Byte,

    /// A fixed 2-bytes of data.
    Fixed2Byte,

    /// A fixed 4-bytes of data.
    Fixed4Byte,

    /// A fixed 8-bytes of data.
    Fixed8Byte,

    /// A fixed 16-bytes of data.
    Fixed16Byte,

    /// A variable-length encoded integer.
    VarInt,

    /// A variable-length encoded integer `n` followed by `n` bytes.
    LengthPrefixed,

    /// A wire-type `w` dual-encoded with a variable-length integer `n`, followed by `n` bytes. The
    /// `n` bytes will be `x` consecutive instances of `w` where `x` is unknown until decoded.
    List,
}

impl WireType {
    //! Bits

    /// Gets the wire type from the low 3-bits of `b`.
    pub fn from_low_3_bits(b: u8) -> Self {
        match b & 0x7 {
            0 => Fixed1Byte,
            1 => Fixed2Byte,
            2 => Fixed4Byte,
            3 => Fixed8Byte,
            4 => Fixed16Byte,
            5 => VarInt,
            6 => LengthPrefixed,
            7 => List,
            _ => unreachable!(),
        }
    }

    /// Gets the wire type from the high 3-bits of `b`.
    pub fn from_high_3_bits(b: u8) -> Self {
        Self::from_low_3_bits(b >> 5)
    }

    /// Converts the wire type to the low 3-bits of a `u8`.
    pub fn to_low_3_bits(&self) -> u8 {
        match self {
            Fixed1Byte => 0,
            Fixed2Byte => 1,
            Fixed4Byte => 2,
            Fixed8Byte => 3,
            Fixed16Byte => 4,
            VarInt => 5,
            LengthPrefixed => 6,
            List => 7,
        }
    }

    /// Converts the wire type to the high 3-bits of a `u8`.
    pub fn to_high_3_bits(&self) -> u8 {
        self.to_low_3_bits() << 5
    }
}

impl Debug for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fixed1Byte => "fixed-1-byte",
                Fixed2Byte => "fixed-2-byte",
                Fixed4Byte => "fixed-4-byte",
                Fixed8Byte => "fixed-8-byte",
                Fixed16Byte => "fixed-16-byte",
                VarInt => "var-int",
                LengthPrefixed => "length-prefixed",
                List => "list",
            }
        )
    }
}
