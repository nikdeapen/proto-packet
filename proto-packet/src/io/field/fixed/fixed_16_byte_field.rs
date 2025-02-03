use std::io::{Error, Write};

use crate::io::{FieldHeader, TagNumber, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};
use uuid::Uuid;

/// A `Fixed16Byte` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Fixed16ByteField {
    tag_number: TagNumber,
    value: [u8; 16],
}

impl Fixed16ByteField {
    //! Constants

    /// The number of fixed bytes. (16)
    pub const FIXED_BYTE_COUNT: usize = 16;

    /// The maximum encoded length of a `Fixed16ByteField`. (6 + 16 = 22)
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + Self::FIXED_BYTE_COUNT;
}

impl Fixed16ByteField {
    //! Construction

    /// Creates a new `Fixed16ByteField`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: [u8; 16]) -> Self {
        Self { tag_number, value }
    }

    /// Creates a new `Fixed16ByteField`.
    #[inline(always)]
    pub const fn from_u128(tag_number: TagNumber, value: &u128) -> Self {
        Self::new(tag_number, value.to_le_bytes())
    }

    /// Creates a new `Fixed16ByteField`.
    #[inline(always)]
    pub const fn from_uuid(tag_number: TagNumber, value: &Uuid) -> Self {
        Self::new(tag_number, *value.as_bytes())
    }
}

impl Fixed16ByteField {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::Fixed16Byte, self.tag_number)
    }
}

impl EncodedLen for Fixed16ByteField {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        Ok(self.field_header().encoded_len()? + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToSlice for Fixed16ByteField {
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
        *target.get_unchecked_mut(header_len + 8) = *self.value.get_unchecked(8);
        *target.get_unchecked_mut(header_len + 9) = *self.value.get_unchecked(9);
        *target.get_unchecked_mut(header_len + 10) = *self.value.get_unchecked(10);
        *target.get_unchecked_mut(header_len + 11) = *self.value.get_unchecked(11);
        *target.get_unchecked_mut(header_len + 12) = *self.value.get_unchecked(12);
        *target.get_unchecked_mut(header_len + 13) = *self.value.get_unchecked(13);
        *target.get_unchecked_mut(header_len + 14) = *self.value.get_unchecked(14);
        *target.get_unchecked_mut(header_len + 15) = *self.value.get_unchecked(15);

        Ok(header_len + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToWrite for Fixed16ByteField {
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
