use enc::{DecodeFromRead, DecodeFromReadPrefix};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use enc::{Error, StreamError};
use proto_packet::io::WireType;
use proto_packet::io::WithTagNumber;
use proto_packet::{Enum, Packet};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// An enum with a single case.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum SingleCase {
    /// // The single case.
    /// One = 1;
    One,
}

impl Packet for SingleCase {
    fn wire_type() -> WireType {
        WireType::VarInt
    }
}

impl Enum for SingleCase {}

impl WithTagNumber for SingleCase {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::One => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl EncodedLen for SingleCase {
    fn encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        let tag_number: u32 = match self {
            Self::One => 1,
        };
        encoded_len += enc::var_int::VarInt32::from(tag_number).encoded_len()?;
        Ok(encoded_len)
    }
}

impl EncodeToSlice for SingleCase {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        let tag_number: u32 = match self {
            Self::One => 1,
        };
        encoded_len += enc::var_int::VarInt32::from(tag_number)
            .encode_to_slice_unchecked(&mut target[encoded_len..])?;
        Ok(encoded_len)
    }
}

impl EncodeToWrite for SingleCase {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;
        let tag_number: u32 = match self {
            Self::One => 1,
        };
        encoded_len += enc::var_int::VarInt32::from(tag_number).encode_to_write(w)?;
        Ok(encoded_len)
    }
}

impl DecodeFromRead for SingleCase {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let first: u8 = enc::read_single_byte(r)?;
        let value: Self = Self::decode_from_read_prefix_with_first_byte(r, first)?;
        debug_assert!(enc::read_optional_byte(r)?.is_none());
        Ok(value)
    }
}

impl DecodeFromReadPrefix for SingleCase {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, StreamError>
    where
        R: Read,
    {
        let tag_number: u32 =
            enc::var_int::VarInt32::decode_from_read_prefix_with_first_byte(r, first)?.value();
        match tag_number {
            1 => Ok(Self::One),
            _ => {
                todo!()
            }
        }
    }
}
