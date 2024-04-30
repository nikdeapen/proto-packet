use std::io::{Error, Read, Write};

use enc::var_int::VarInt32;
use enc::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::WireType;

/// A message field header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FieldHeader {
    wire_type: WireType,
    field_number: u32,
}

impl FieldHeader {
    //! Construction

    /// Creates a new field header.
    #[inline(always)]
    pub fn new(wire_type: WireType, field_number: u32) -> Self {
        debug_assert!(field_number != 0);

        Self {
            wire_type,
            field_number,
        }
    }
}

impl FieldHeader {
    //! Constants

    /// The maximum field number for field headers that fit in a single byte. (31)
    pub const MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER: u32 = 0x1F;

    /// The maximum encoded length of a field header.
    pub const MAX_ENCODED_LEN: usize = 1 + VarInt32::MAX_ENCODED_LEN;
}

impl EncodedLen for FieldHeader {
    fn encoded_len(&self) -> usize {
        if self.field_number <= Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER {
            1
        } else {
            let extra: u32 = self.field_number - Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER - 1;
            let extra: usize = VarInt32::from(extra).encoded_len();
            1 + extra
        }
    }
}

impl EncodeToSlice for FieldHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        if self.field_number <= Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER {
            *target.get_unchecked_mut(0) =
                self.wire_type.to_high_three_bits() | (self.field_number as u8);
            1
        } else {
            *target.get_unchecked_mut(0) = self.wire_type.to_high_three_bits();
            let extra: u32 = self.field_number - Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER - 1;
            let extra: usize = VarInt32::from(extra).encode_to_slice_unchecked(&mut target[1..]);
            1 + extra
        }
    }
}

impl EncodeToWrite for FieldHeader {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer) };
        w.write_all(&mut buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl DecodeFromReadPrefix for FieldHeader {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire_type: WireType = WireType::from_high_three_bits(first);
        let field_number: u32 = (first as u32) & Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER;
        if field_number != 0 {
            Ok(Self::new(wire_type, field_number))
        } else {
            let extra: u32 = VarInt32::decode_from_read_prefix(r)?.value;
            let field_number: u32 = Self::MAX_SINGLE_BYTE_HEADER_FIELD_NUMBER + 1 + extra;
            Ok(Self::new(wire_type, field_number))
        }
    }
}
