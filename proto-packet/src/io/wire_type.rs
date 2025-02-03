use std::io::{Error, Read};

use enc::var_int::VarIntSize;
use enc::DecodeFromReadPrefix;

use crate::io::WireType::*;

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

    /// A wire-type `w` duel-encoded with a variable-length integer `n` followed by `n` bytes. The
    /// following `n` bytes will be `x` consecutive instances of `w` where `x` is unknown.
    List,
}

impl WireType {
    //! Bits

    /// Converts the wire type from the low 3-bits of `b`.
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

    /// Converts the wire type from the high 3-bits of `b`.
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

    /// Decodes a `Fixed2Byte` value.
    pub fn decode_fixed_2_byte<R>(r: &mut R) -> Result<[u8; 2], Error>
    where
        R: Read,
    {
        let mut b: [u8; 2] = [0u8; 2];
        r.read_exact(&mut b[..])?;
        Ok(b)
    }

    /// Decodes a `Fixed4Byte` value.
    pub fn decode_fixed_4_byte<R>(r: &mut R) -> Result<[u8; 4], Error>
    where
        R: Read,
    {
        let mut b: [u8; 4] = [0u8; 4];
        r.read_exact(&mut b[..])?;
        Ok(b)
    }

    /// Decodes a `Fixed8Byte` value.
    pub fn decode_fixed_8_byte<R>(r: &mut R) -> Result<[u8; 8], Error>
    where
        R: Read,
    {
        let mut b: [u8; 8] = [0u8; 8];
        r.read_exact(&mut b[..])?;
        Ok(b)
    }

    /// Decodes a `Fixed16Byte` value.
    pub fn decode_fixed_16_byte<R>(r: &mut R) -> Result<[u8; 16], Error>
    where
        R: Read,
    {
        let mut b: [u8; 16] = [0u8; 16];
        r.read_exact(&mut b[..])?;
        Ok(b)
    }

    /// Decodes a `LengthPrefixed` value.
    pub fn decode_length_prefixed<R>(r: &mut R) -> Result<Vec<u8>, Error>
    where
        R: Read,
    {
        let len_prefix: usize = VarIntSize::decode_from_read_prefix(r)?.value;
        let mut buffer: Vec<u8> = vec![0u8; len_prefix];
        r.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
