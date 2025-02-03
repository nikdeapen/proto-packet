use std::io::{Error, Write};

use enc::var_int::VarIntSize;
use enc::Error::IntegerOverflow;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::{FieldHeader, TagNumber, WireType};

/// A `LengthPrefixed` bytes field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct BytesField<'a> {
    tag_number: TagNumber,
    value: &'a [u8],
}

impl<'a> BytesField<'a> {
    //! Construction

    /// Creates a new `BytesField`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: &'a [u8]) -> Self {
        Self { tag_number, value }
    }

    /// Creates a new `BytesField`.
    #[inline(always)]
    pub const fn from_string(tag_number: TagNumber, value: &'a str) -> Self {
        Self::new(tag_number, value.as_bytes())
    }
}

impl<'a> BytesField<'a> {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(WireType::LengthPrefixed, self.tag_number)
    }
}

impl<'a> EncodedLen for BytesField<'a> {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encoded_len()?;
        let prefix_len: usize = VarIntSize::from(self.value.len()).encoded_len()?;
        (header_len + prefix_len)
            .checked_add(self.value.len())
            .ok_or(IntegerOverflow)
    }
}

impl<'a> EncodeToSlice for BytesField<'a> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encode_to_slice_unchecked(target)?;
        let prefix_len: usize = VarIntSize::from(self.value.len())
            .encode_to_slice_unchecked(&mut target[header_len..])?;
        (&mut target[(header_len + prefix_len)..(header_len + prefix_len + self.value.len())])
            .copy_from_slice(self.value);
        Ok(header_len + prefix_len + self.value.len())
    }
}

impl<'a> EncodeToWrite for BytesField<'a> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let encoded_len: usize = self.encoded_len()?;
        self.field_header().encode_to_write(w)?;
        VarIntSize::from(self.value.len()).encode_to_write(w)?;
        w.write_all(self.value)?;
        Ok(encoded_len)
    }
}
