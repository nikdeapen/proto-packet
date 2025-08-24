use crate::io::DecodingError;
use crate::io::WireType::*;
use enc::var_int::VarIntSize;
use enc::DecodeFromReadPrefix;
use std::fmt::{Display, Formatter};
use std::io::Read;

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
    /// `x` sequential instances of `w` where `x` is unknown until the list is decoded.
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

impl WireType {
    //! Decode

    /// Decodes a `[]u8` value from the `Read` prefix given the `first` byte.
    pub fn decode_bytes<R>(r: &mut R, first: u8) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(|e| DecodingError::from_length_prefix_error(e))?
            .value();
        let mut result: Vec<u8> = vec![0; prefix];
        r.read_exact(&mut result)
            .map_err(|e| DecodingError::Source(e))?;
        Ok(result)
    }
}

impl Display for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
