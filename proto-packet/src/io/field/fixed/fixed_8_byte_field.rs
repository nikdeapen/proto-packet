use std::io::{Error, Write};

use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::{FieldHeader, TagNumber, WireType};

/// A `Fixed8Byte` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Fixed8ByteField {
    tag_number: TagNumber,
    value: [u8; 8],
}

impl Fixed8ByteField {
    //! Constants

    /// The number of fixed bytes. (8)
    pub const FIXED_BYTE_COUNT: usize = 8;

    /// The maximum encoded length of a `Fixed8ByteField`. (6 + 8 = 14)
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + Self::FIXED_BYTE_COUNT;
}

impl Fixed8ByteField {
    //! Construction

    /// Creates a new `Fixed8ByteField`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: [u8; 8]) -> Self {
        Self { tag_number, value }
    }
}

impl Fixed8ByteField {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::Fixed8Byte, self.tag_number)
    }

    /// Creates a new `Fixed8ByteField`.
    #[inline(always)]
    pub const fn from_u64(tag_number: TagNumber, value: &u64) -> Self {
        Self::new(tag_number, value.to_le_bytes())
    }
}

impl EncodedLen for Fixed8ByteField {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        Ok(self.field_header().encoded_len()? + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToSlice for Fixed8ByteField {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encode_to_slice_unchecked(target)?;

        *target.get_unchecked_mut(header_len) = *self.value.get_unchecked(0);
        *target.get_unchecked_mut(header_len + 1) = *self.value.get_unchecked(1);
        *target.get_unchecked_mut(header_len + 2) = *self.value.get_unchecked(2);
        *target.get_unchecked_mut(header_len + 3) = *self.value.get_unchecked(3);
        *target.get_unchecked_mut(header_len + 4) = *self.value.get_unchecked(4);
        *target.get_unchecked_mut(header_len + 5) = *self.value.get_unchecked(5);
        *target.get_unchecked_mut(header_len + 6) = *self.value.get_unchecked(6);
        *target.get_unchecked_mut(header_len + 7) = *self.value.get_unchecked(7);

        Ok(header_len + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToWrite for Fixed8ByteField {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
        w.write_all(&buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}
