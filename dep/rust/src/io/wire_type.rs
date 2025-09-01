use crate::io::DecodingError;
use crate::io::DecodingError::Source;
use crate::io::WireType::*;
use enc::var_int::VarIntSize;
use enc::{read_single_byte, DecodeFromReadPrefix, EncodeToWrite};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Read, Take, Write};

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

    /// Decodes a `Fixed2Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_2_byte<R>(r: &mut R, first: u8) -> Result<[u8; 2], DecodingError>
    where
        R: Read,
    {
        Ok([first, read_single_byte(r).map_err(|e| Source(e))?])
    }

    /// Decodes a `Fixed4Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_4_byte<R>(r: &mut R, first: u8) -> Result<[u8; 4], DecodingError>
    where
        R: Read,
    {
        let mut buffer: [u8; 4] = [0u8; 4];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..]).map_err(|e| Source(e))?;
        Ok(buffer)
    }

    /// Decodes a `Fixed8Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_8_byte<R>(r: &mut R, first: u8) -> Result<[u8; 8], DecodingError>
    where
        R: Read,
    {
        let mut buffer: [u8; 8] = [0u8; 8];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..]).map_err(|e| Source(e))?;
        Ok(buffer)
    }

    /// Decodes a `Fixed16Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_16_byte<R>(r: &mut R, first: u8) -> Result<[u8; 16], DecodingError>
    where
        R: Read,
    {
        let mut buffer: [u8; 16] = [0u8; 16];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..]).map_err(|e| Source(e))?;
        Ok(buffer)
    }

    /// Decodes a `[]u8` value from the `Read` prefix given the `first` byte.
    pub fn decode_bytes<R>(r: &mut R, first: u8) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(|e| DecodingError::from_length_prefix_error(e))?
            .value();
        let mut result: Vec<u8> = vec![0; prefix];
        r.read_exact(&mut result).map_err(|e| Source(e))?;
        Ok(result)
    }
}

impl WireType {
    //! Transfer

    /// Transfers the wire type data from the `Read` to the `Write`.
    pub fn transfer<R, W>(&self, r: &mut R, w: &mut W) -> Result<(), io::Error>
    where
        R: Read,
        W: Write,
    {
        let first: u8 = read_single_byte(r)?;
        self.transfer_with_first_byte(r, w, first)
    }

    /// Transfers wire type from data the `Read` to the `Write` with the `first` byte.
    pub fn transfer_with_first_byte<R, W>(
        &self,
        r: &mut R,
        w: &mut W,
        first: u8,
    ) -> Result<(), io::Error>
    where
        R: Read,
        W: Write,
    {
        match self {
            Fixed1Byte => w.write_all(&[first])?,
            Fixed2Byte => {
                let second: u8 = read_single_byte(r)?;
                w.write_all(&[first, second])?;
            }
            Fixed4Byte => {
                let mut b: [u8; 4] = [0u8; 4];
                b[0] = first;
                r.read_exact(&mut b[1..])?;
                w.write_all(&b)?;
            }
            Fixed8Byte => {
                let mut b: [u8; 8] = [0u8; 8];
                b[0] = first;
                r.read_exact(&mut b[1..])?;
                w.write_all(&b)?;
            }
            Fixed16Byte => {
                let mut b: [u8; 16] = [0u8; 16];
                b[0] = first;
                r.read_exact(&mut b[1..])?;
                w.write_all(&b)?;
            }
            VarInt => {
                let mut current: u8 = first;
                loop {
                    w.write_all(&[current])?;
                    if current & 0x10 == 0 {
                        break;
                    }
                    current = read_single_byte(r)?;
                }
            }
            LengthPrefixed => {
                let prefix: VarIntSize =
                    VarIntSize::decode_from_read_prefix_with_first_byte(r, first)?;
                prefix.encode_to_write(w)?;
                const _: () = debug_assert!(usize::BITS <= u64::BITS);
                let mut r: Take<&mut R> = r.take(prefix.value() as u64);
                io::copy(&mut r, w)?;
            }
            List => {}
        }
        Ok(())
    }
}

impl Display for WireType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
