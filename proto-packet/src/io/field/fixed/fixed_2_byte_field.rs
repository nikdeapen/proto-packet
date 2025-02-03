use std::io::{Error, Write};

use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::{FieldHeader, TagNumber, WireType};

/// A `Fixed2Byte` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Fixed2ByteField {
    tag_number: TagNumber,
    value: [u8; 2],
}

impl Fixed2ByteField {
    //! Constants

    /// The number of fixed bytes. (2)
    pub const FIXED_BYTE_COUNT: usize = 2;

    /// The maximum encoded length of a `Fixed2ByteField`. (6 + 2 = 22)
    pub const MAX_ENCODED_LEN: usize = FieldHeader::MAX_ENCODED_LEN + Self::FIXED_BYTE_COUNT;
}

impl Fixed2ByteField {
    //! Construction

    /// Creates a new `Fixed2ByteField`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: [u8; 2]) -> Self {
        Self { tag_number, value }
    }

    /// Creates a new `Fixed2ByteField`.
    #[inline(always)]
    pub const fn from_u16(tag_number: TagNumber, value: &u16) -> Self {
        Self::new(tag_number, value.to_le_bytes())
    }
}

impl Fixed2ByteField {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::Fixed2Byte, self.tag_number)
    }
}

impl EncodedLen for Fixed2ByteField {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        Ok(self.field_header().encoded_len()? + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToSlice for Fixed2ByteField {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encode_to_slice_unchecked(target)?;

        *target.get_unchecked_mut(header_len) = *self.value.get_unchecked(0);
        *target.get_unchecked_mut(header_len + 1) = *self.value.get_unchecked(1);

        Ok(header_len + Self::FIXED_BYTE_COUNT)
    }
}

impl EncodeToWrite for Fixed2ByteField {
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
