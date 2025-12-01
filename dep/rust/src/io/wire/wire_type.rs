use std::fmt::{Display, Formatter};

/// A wire type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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

    /// A wire-type `w` dual-encoded with a var-int `n`, followed by `n` bytes. The `n` bytes are
    /// `x` sequential instances of `w` where `x` is unknown (until the list is decoded).
    List,
}

impl Display for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
