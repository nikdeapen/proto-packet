use std::fmt::{Display, Formatter};

/// A wire type.
///
/// The discriminants are the on-wire encoding of the wire type — they appear in the high or low 3 bits of
/// header bytes throughout the format. Do not reorder or change them without breaking wire compatibility.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum WireType {
    /// A fixed 1-byte of data.
    Fixed1Byte = 0,

    /// A fixed 2-bytes of data.
    Fixed2Byte = 1,

    /// A fixed 4-bytes of data.
    Fixed4Byte = 2,

    /// A fixed 8-bytes of data.
    Fixed8Byte = 3,

    /// A fixed 16-bytes of data.
    Fixed16Byte = 4,

    /// A variable-length encoded integer.
    VarInt = 5,

    /// A variable-length encoded integer `n` followed by `n` bytes.
    LengthPrefixed = 6,

    /// A wire-type `w` dual-encoded with a var-primitive `n`, followed by `n` bytes. The `n` bytes are
    /// `x` sequential instances of `w` where `x` is unknown (until the list is decoded).
    List = 7,
}

impl Display for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
