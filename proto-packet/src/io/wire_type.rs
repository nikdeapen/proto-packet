use crate::io::WireType::*;

/// A wire-type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum WireType {
    /// Fixed length of 1 byte.
    Fixed1Byte,

    /// Fixed length of 2 bytes.
    Fixed2Bytes,

    /// Fixed length of 4 bytes.
    Fixed4Bytes,

    /// Fixed length of 8 bytes.
    Fixed8Bytes,

    /// Fixed length of 16 bytes.
    Fixed16Bytes,

    /// A variable-length encoded integer.
    VarInt,

    /// A variable-length encoded integer `n` followed by `n` bytes.
    LengthPrefixed,

    /// A wire-type with a variable-length encoded integer `n` followed by `n` bytes.
    List,
}

impl WireType {
    //! Bits

    /// Gets the wire-type for the high three bits of `b`.
    #[inline(always)]
    pub fn from_high_three_bits(b: u8) -> Self {
        Self::from_low_three_bits(b >> 5)
    }

    /// Gets the wire-type for the low three bits of `b`.
    pub fn from_low_three_bits(b: u8) -> Self {
        match b & 0x7 {
            0 => Fixed1Byte,
            1 => Fixed2Bytes,
            2 => Fixed4Bytes,
            3 => Fixed8Bytes,
            4 => Fixed16Bytes,
            5 => VarInt,
            6 => LengthPrefixed,
            7 => List,
            _ => unreachable!(),
        }
    }

    /// Converts the wire-type to the high three bits of a byte.
    #[inline(always)]
    pub fn to_high_three_bits(&self) -> u8 {
        self.to_low_three_bits() << 5
    }

    /// Converts the wire-type to the low three bits of a byte.
    pub fn to_low_three_bits(&self) -> u8 {
        match *self {
            Fixed1Byte => 0,
            Fixed2Bytes => 1,
            Fixed4Bytes => 2,
            Fixed8Bytes => 3,
            Fixed16Bytes => 4,
            VarInt => 5,
            LengthPrefixed => 6,
            List => 7,
        }
    }
}
